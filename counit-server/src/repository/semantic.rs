use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::{Arc, RwLock};

use futures::{stream, StreamExt, TryStreamExt};
use ndarray::Axis;
use ort::{
    Environment,
    ExecutionProvider, GraphOptimizationLevel, LoggingLevel, SessionBuilder, tensor::InputTensor};
use ort::tensor::{FromArray, OrtOwnedTensor};
use qdrant_client::{
    prelude::{QdrantClient, QdrantClientConfig},
    qdrant::{
        CollectionOperationResponse, CreateCollection,
        Distance, FieldCondition, FieldType,
        Filter, Match, PointId, r#match::MatchValue, ScoredPoint, SearchPoints, VectorParams,
        vectors_config, VectorsConfig, with_payload_selector, with_vectors_selector,
        WithPayloadSelector, WithVectorsSelector,
    },
};
use qdrant_client::qdrant::PointStruct;
use thiserror::Error;
use tracing::{debug, info, trace};

use crate::configuration::Configuration;
use crate::repository::cache_key;
use crate::repository::payload::CodePayload;
use crate::repository::semantic_query::SemanticQuery;

#[derive(Clone)]
pub struct Semantic {
    qdrant: Arc<QdrantClient>,
    tokenizer: Arc<tokenizers::Tokenizer>,
    session: Arc<ort::Session>,
    config: Arc<Configuration>,
}

#[derive(Error, Debug)]
pub enum SemanticError {
    /// Represents failure to initialize Qdrant client
    #[error("Qdrant initialization failed. Is Qdrant running on `qdrant-url`?")]
    QdrantInitializationError,

    #[error("ONNX runtime error")]
    OnnxRuntimeError {
        #[from]
        error: ort::OrtError,
    },

    #[error("semantic error")]
    Anyhow {
        #[from]
        error: anyhow::Error,
    },
}


pub(crate) const COLLECTION_NAME: &str = "documents";
pub(crate) const EMBEDDING_DIM: usize = 384;

pub type Embedding = Vec<f32>;

fn collection_config() -> CreateCollection {
    CreateCollection {
        collection_name: COLLECTION_NAME.to_string(),
        vectors_config: Some(VectorsConfig {
            config: Some(vectors_config::Config::Params(VectorParams {
                size: EMBEDDING_DIM as u64,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }
}

/// Initialize the `ORT_DYLIB_PATH` variable, consumed by the `ort` crate.
///
/// This doesn't do anything on Windows, as tauri on Windows will automatically bundle any `.dll`
/// files found in the `target/$profile` folder. The `ort` crate by default will also copy the
/// built dynamic library over to the `target/$profile` folder, when using the download strategy.
fn init_ort_dylib(dylib_dir: impl AsRef<Path>) {
    #[cfg(not(windows))]
    {
        #[cfg(target_os = "linux")]
            let lib_name = "libonnxruntime.so";
        #[cfg(target_os = "macos")]
            let lib_name = "libonnxruntime.dylib";

        let ort_dylib_path = dylib_dir.as_ref().join(lib_name);

        if env::var("ORT_DYLIB_PATH").is_err() {
            env::set_var("ORT_DYLIB_PATH", ort_dylib_path);
        }
    }
}


impl Semantic {
    pub async fn initialize(
        model_dir: &Path,
        qdrant_url: &str,
        config: Arc<Configuration>,
    ) -> Result<Self, SemanticError> {
        let qdrant = QdrantClient::new(Some(QdrantClientConfig::from_url(qdrant_url))).unwrap();

        match qdrant.has_collection(COLLECTION_NAME).await {
            Ok(false) => {
                let CollectionOperationResponse { result, time } = qdrant
                    .create_collection(&collection_config())
                    .await
                    .unwrap();

                debug!(
                    time,
                    created = result,
                    name = COLLECTION_NAME,
                    "created qdrant collection"
                );

                assert!(result);
            }
            Ok(true) => {}
            Err(_) => return Err(SemanticError::QdrantInitializationError),
        }

        qdrant
            .create_field_index(COLLECTION_NAME, "repo_ref", FieldType::Text, None, None)
            .await?;
        qdrant
            .create_field_index(COLLECTION_NAME, "content_hash", FieldType::Text, None, None)
            .await?;
        qdrant
            .create_field_index(COLLECTION_NAME, "branches", FieldType::Text, None, None)
            .await?;
        qdrant
            .create_field_index(
                COLLECTION_NAME,
                "relative_path",
                FieldType::Text,
                None,
                None,
            )
            .await?;

        if let Some(dylib_dir) = config.dylib_dir.as_ref() {
            init_ort_dylib(dylib_dir);
        }

        let environment = Arc::new(
            Environment::builder()
                .with_name("Encode")
                .with_log_level(LoggingLevel::Warning)
                .with_execution_providers([ExecutionProvider::cpu()])
                .with_telemetry(false)
                .build()?,
        );

        let threads = if let Ok(v) = std::env::var("NUM_OMP_THREADS") {
            str::parse(&v).unwrap_or(1)
        } else {
            1
        };

        Ok(Self {
            qdrant: qdrant.into(),
            tokenizer: tokenizers::Tokenizer::from_file(model_dir.join("tokenizer.json"))
                .unwrap()
                .into(),
            session: SessionBuilder::new(&environment)?
                .with_optimization_level(GraphOptimizationLevel::Level3)?
                .with_intra_threads(threads)?
                .with_model_from_file(model_dir.join("model.onnx"))?
                .into(),
            config,
        })
    }

    pub async fn health_check(&self) -> anyhow::Result<()> {
        self.qdrant.health_check().await?;
        Ok(())
    }

    pub fn embed(&self, sequence: &str) -> anyhow::Result<Embedding> {
        let tokenizer_output = self.tokenizer.encode(sequence, true).unwrap();

        let input_ids = tokenizer_output.get_ids();
        let attention_mask = tokenizer_output.get_attention_mask();
        let token_type_ids = tokenizer_output.get_type_ids();
        let length = input_ids.len();
        trace!("embedding {} tokens {:?}", length, sequence);

        let inputs_ids_array = ndarray::Array::from_shape_vec(
            (1, length),
            input_ids.iter().map(|&x| x as i64).collect(),
        )?;

        let attention_mask_array = ndarray::Array::from_shape_vec(
            (1, length),
            attention_mask.iter().map(|&x| x as i64).collect(),
        )?;

        let token_type_ids_array = ndarray::Array::from_shape_vec(
            (1, length),
            token_type_ids.iter().map(|&x| x as i64).collect(),
        )?;

        let outputs = self.session.run([
            InputTensor::from_array(inputs_ids_array.into_dyn()),
            InputTensor::from_array(attention_mask_array.into_dyn()),
            InputTensor::from_array(token_type_ids_array.into_dyn()),
        ])?;

        let output_tensor: OrtOwnedTensor<f32, _> = outputs[0].try_extract().unwrap();
        let sequence_embedding = &*output_tensor.view();
        let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();

        Ok(pooled.to_owned().as_slice().unwrap().to_vec())
    }

    pub async fn search_with<'a>(
        &self,
        parsed_query: &SemanticQuery<'a>,
        vector: Embedding,
        limit: u64,
        offset: u64,
        threshold: f32,
    ) -> anyhow::Result<Vec<ScoredPoint>> {
        let response = self
            .qdrant
            .search_points(&SearchPoints {
                limit,
                vector,
                collection_name: COLLECTION_NAME.to_string(),
                offset: Some(offset),
                score_threshold: Some(threshold),
                with_payload: Some(WithPayloadSelector {
                    selector_options: Some(with_payload_selector::SelectorOptions::Enable(true)),
                }),
                filter: Some(Filter {
                    must: build_conditions(parsed_query),
                    ..Default::default()
                }),
                with_vectors: Some(WithVectorsSelector {
                    selector_options: Some(with_vectors_selector::SelectorOptions::Enable(true)),
                }),
                ..Default::default()
            })
            .await?;

        Ok(response.result)
    }

    pub async fn search<'a>(
        &self,
        parsed_query: &SemanticQuery<'a>,
        limit: u64,
        offset: u64,
        threshold: f32,
        retrieve_more: bool,
    ) -> anyhow::Result<Vec<CodePayload>> {
        let Some(query) = parsed_query.target() else {
            anyhow::bail!("no search target for query");
        };
        let vector = self.embed(&query)?;

        // TODO: Remove the need for `retrieve_more`. It's here because:
        // In /q `limit` is the maximum number of results returned (the actual number will often be lower due to deduplication)
        // In /answer we want to retrieve `limit` results exactly
        let results = self
            .search_with(
                parsed_query,
                vector.clone(),
                if retrieve_more { limit * 2 } else { limit }, // Retrieve double `limit` and deduplicate
                offset,
                threshold,
            )
            .await
            .map(|raw| {
                raw.into_iter()
                    .map(CodePayload::from_qdrant)
                    .collect::<Vec<_>>()
            })?;

        Ok(deduplicate_snippets(results, vector, limit))
    }

    pub async fn batch_search<'a>(
        &self,
        parsed_queries: &[&SemanticQuery<'a>],
        limit: u64,
        offset: u64,
        threshold: f32,
        retrieve_more: bool,
    ) -> anyhow::Result<Vec<CodePayload>> {
        if parsed_queries.iter().any(|q| q.target().is_none()) {
            anyhow::bail!("no search target for query");
        };

        let vectors = parsed_queries
            .iter()
            .map(|q| self.embed(&q.target().unwrap()))
            .collect::<anyhow::Result<Vec<_>>>()?;

        tracing::trace!(?parsed_queries, "performing qdrant batch search");

        let result = self
            .batch_search_with(
                parsed_queries,
                vectors.clone(),
                if retrieve_more { limit * 2 } else { limit }, // Retrieve double `limit` and deduplicate
                offset,
                threshold,
            )
            .await;

        tracing::trace!(?result, "qdrant batch search returned");

        let results = result?
            .into_iter()
            .map(CodePayload::from_qdrant)
            .collect::<Vec<_>>();

        // deduplicate with mmr with respect to the mean of query vectors
        // TODO: implement a more robust multi-vector deduplication strategy
        let target_vector = mean_pool(vectors);
        Ok(deduplicate_snippets(results, target_vector, limit))
    }

    pub async fn batch_search_with<'a>(
        &self,
        parsed_queries: &[&SemanticQuery<'a>],
        vectors: Vec<Embedding>,
        limit: u64,
        offset: u64,
        threshold: f32,
    ) -> anyhow::Result<Vec<ScoredPoint>> {
        // FIXME: This method uses `search_points` internally, and not `search_batch_points`. It's
        // not clear why, but it seems that the `batch` variant of the `qdrant` calls leads to
        // HTTP2 errors on some deployment configurations. A typical example error:
        //
        // ```
        // hyper::proto::h2::client: client response error: stream error received: stream no longer needed
        // ```
        //
        // Given that qdrant uses `tonic`, this may be a `tonic` issue, possibly similar to:
        // https://github.com/hyperium/tonic/issues/222

        // Queries should contain the same filters, so we get the first one
        let parsed_query = parsed_queries.first().unwrap();
        let filters = &build_conditions(parsed_query);

        let responses = stream::iter(vectors.into_iter())
            .map(|vector| async move {
                let points = SearchPoints {
                    limit,
                    vector,
                    collection_name: COLLECTION_NAME.to_string(),
                    offset: Some(offset),
                    score_threshold: Some(threshold),
                    with_payload: Some(WithPayloadSelector {
                        selector_options: Some(with_payload_selector::SelectorOptions::Enable(
                            true,
                        )),
                    }),
                    filter: Some(Filter {
                        must: filters.clone(),
                        ..Default::default()
                    }),
                    with_vectors: Some(WithVectorsSelector {
                        selector_options: Some(with_vectors_selector::SelectorOptions::Enable(
                            true,
                        )),
                    }),
                    ..Default::default()
                };

                self.qdrant.search_points(&points).await
            })
            .buffered(10)
            .try_collect::<Vec<_>>()
            .await?;

        Ok(responses.into_iter().flat_map(|r| r.result).collect())
    }


    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(skip(self, repo_name, buffer))]
    pub async fn insert_points_for_buffer(
        &self,
        repo_name: &str,
        repo_ref: &str,
        relative_path: &str,
        buffer: &str,
    ) -> anyhow::Result<()> {
        let embedded = self.embed(buffer)?;
        let new: RwLock<Vec<PointStruct>> = Default::default();

        let payload = CodePayload {
            lang: "java".to_string(),
            repo_name: repo_name.to_string(),
            repo_ref: repo_ref.to_string(),
            relative_path: relative_path.to_string(),
            content_hash: "".to_string(),
            text: buffer.to_string(),
            origin_text: buffer.to_string(),
            start_line: 0,
            end_line: 0,
            start_byte: 0,
            end_byte: 0,
            branches: vec![],
            id: None,
            embedding: None,
            score: None,
        };

        let id = cache_key(buffer);

        new.write().unwrap().push(PointStruct {
            id: Some(PointId::from(id)),
            vectors: Some(embedded.into()),
            payload: payload.into_qdrant(),
        });

        let point: Vec<_> = std::mem::take(new.write().unwrap().as_mut());

        self.qdrant
            .upsert_points_blocking(COLLECTION_NAME, point, None)
            .await?;

        Ok(())
    }
}

pub fn deduplicate_snippets(
    mut all_snippets: Vec<CodePayload>,
    query_embedding: Embedding,
    output_count: u64,
) -> Vec<CodePayload> {
    all_snippets = filter_overlapping_snippets(all_snippets);

    let idxs = {
        let lambda = 0.5;
        let k = output_count; // number of snippets
        let embeddings = all_snippets
            .iter()
            .map(|s| s.embedding.as_deref().unwrap())
            .collect::<Vec<_>>();
        let languages = all_snippets
            .iter()
            .map(|s| s.lang.as_ref())
            .collect::<Vec<_>>();
        let paths = all_snippets
            .iter()
            .map(|s| s.relative_path.as_ref())
            .collect::<Vec<_>>();
        deduplicate_with_mmr(
            &query_embedding,
            &embeddings,
            &languages,
            &paths,
            lambda,
            k as usize,
        )
    };

    info!("preserved idxs after MMR are {:?}", idxs);

    all_snippets
        .drain(..)
        .enumerate()
        .filter_map(|(ref i, payload)| {
            if idxs.contains(i) {
                Some(payload)
            } else {
                None
            }
        })
        .collect()
}

fn filter_overlapping_snippets(mut snippets: Vec<CodePayload>) -> Vec<CodePayload> {
    snippets.sort_by(|a, b| {
        a.relative_path
            .cmp(&b.relative_path)
            .then(a.start_line.cmp(&b.start_line))
    });

    snippets = snippets
        .into_iter()
        .fold(Vec::<CodePayload>::new(), |mut deduped_snippets, snippet| {
            if let Some(prev) = deduped_snippets.last_mut() {
                if prev.relative_path == snippet.relative_path
                    && prev.end_line >= snippet.start_line
                {
                    debug!(
                        "Filtering overlapping snippets. End: {:?} - Start: {:?} from {:?}",
                        prev.end_line, snippet.start_line, prev.relative_path
                    );
                    return deduped_snippets;
                }
            }
            deduped_snippets.push(snippet);
            deduped_snippets
        });

    snippets.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    snippets
}

// returns a list of indices to preserve from `snippets`
//
// query_embedding: the embedding of the query terms
// embeddings: the list of embeddings to select from
// lambda: MMR is a weighted selection of two opposing factors:
//    - relevance to the query
//    - "novelty" or, the measure of how minimal the similarity is
//      to existing documents in the selection
//      The value of lambda skews the weightage in favor of either relevance or novelty.
//    - we add a language diversity factor to the score to encourage a range of langauges in the results
//    - we also add a path diversity factor to the score to encourage a range of paths in the results
//  k: the number of embeddings to select
pub fn deduplicate_with_mmr(
    query_embedding: &[f32],
    embeddings: &[&[f32]],
    languages: &[&str],
    paths: &[&str],
    lambda: f32,
    k: usize,
) -> Vec<usize> {
    let mut idxs = vec![];
    let mut lang_counts = HashMap::new();
    let mut path_counts = HashMap::new();

    if embeddings.len() < k {
        return (0..embeddings.len()).collect();
    }

    while idxs.len() < k {
        let mut best_score = f32::NEG_INFINITY;
        let mut idx_to_add = None;

        for (i, emb) in embeddings.iter().enumerate() {
            if idxs.contains(&i) {
                continue;
            }
            let first_part = cosine_similarity(query_embedding, emb);
            let mut second_part = 0.;
            for j in idxs.iter() {
                let cos_sim = cosine_similarity(emb, embeddings[*j]);
                if cos_sim > second_part {
                    second_part = cos_sim;
                }
            }
            let mut equation_score = lambda * first_part - (1. - lambda) * second_part;

            // MMR + (1/2)^n where n is the number of times a language has been selected
            let lang_count = lang_counts.get(languages[i]).unwrap_or(&0);
            equation_score += 0.5_f32.powi(*lang_count);

            // MMR + (3/4)^n where n is the number of times a path has been selected
            let path_count = path_counts.get(paths[i]).unwrap_or(&0);
            equation_score += 0.75_f32.powi(*path_count);

            if equation_score > best_score {
                best_score = equation_score;
                idx_to_add = Some(i);
            }
        }
        if let Some(i) = idx_to_add {
            idxs.push(i);
            *lang_counts.entry(languages[i]).or_insert(0) += 1;
            *path_counts.entry(paths[i]).or_insert(0) += 1;
        }
    }
    idxs
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai * bi).sum()
}

fn norm(a: &[f32]) -> f32 {
    dot(a, a)
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    dot(a, b) / (norm(a) * norm(b))
}

fn build_conditions(query: &SemanticQuery<'_>) -> Vec<qdrant_client::qdrant::Condition> {
    let repo_filter = {
        let conditions = query
            .repos()
            .map(|r| {
                if r.contains('/') && !r.starts_with("github.com/") {
                    format!("github.com/{r}")
                } else {
                    r.to_string()
                }
            })
            .map(|r| make_kv_keyword_filter("repo_name", r.as_ref()).into())
            .collect::<Vec<_>>();
        // one of the above repos should match
        if conditions.is_empty() {
            None
        } else {
            Some(Filter {
                should: conditions,
                ..Default::default()
            })
        }
    };

    let path_filter = {
        let conditions = query
            .paths()
            .map(|r| make_kv_text_filter("relative_path", r.as_ref()).into())
            .collect::<Vec<_>>();
        if conditions.is_empty() {
            None
        } else {
            Some(Filter {
                should: conditions,
                ..Default::default()
            })
        }
    };

    let lang_filter = {
        let conditions = query
            .langs()
            .map(|l| make_kv_keyword_filter("lang", l.as_ref()).into())
            .collect::<Vec<_>>();
        // one of the above langs should match
        if conditions.is_empty() {
            None
        } else {
            Some(Filter {
                should: conditions,
                ..Default::default()
            })
        }
    };

    let branch_filter = {
        let conditions = query
            .branch()
            .map(|l| make_kv_keyword_filter("branches", l.as_ref()).into())
            .collect::<Vec<_>>();

        if conditions.is_empty() {
            None
        } else {
            Some(Filter {
                should: conditions,
                ..Default::default()
            })
        }
    };

    let filters: Vec<_> = [repo_filter, path_filter, lang_filter, branch_filter]
        .into_iter()
        .flatten()
        .map(Into::into)
        .collect();

    filters
}

// Substring match filter
fn make_kv_text_filter(key: &str, value: &str) -> FieldCondition {
    let key = key.to_owned();
    let value = value.to_owned();
    FieldCondition {
        key,
        r#match: Some(Match {
            match_value: MatchValue::Text(value).into(),
        }),
        ..Default::default()
    }
}

// Exact match filter
pub(crate) fn make_kv_keyword_filter(key: &str, value: &str) -> FieldCondition {
    let key = key.to_owned();
    let value = value.to_owned();
    FieldCondition {
        key,
        r#match: Some(Match {
            match_value: MatchValue::Keyword(value).into(),
        }),
        ..Default::default()
    }
}

// Calculate the element-wise mean of the embeddings
fn mean_pool(embeddings: Vec<Vec<f32>>) -> Vec<f32> {
    let len = embeddings.len() as f32;
    let mut result = vec![0.0; EMBEDDING_DIM];
    for embedding in embeddings {
        for (i, v) in embedding.iter().enumerate() {
            result[i] += v;
        }
    }
    result.iter_mut().for_each(|v| *v /= len);
    result
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;
    use crate::configuration::Configuration;
    use crate::repository::semantic::Semantic;

    #[tokio::test]
    async fn test_mmr() {
        let model_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent()
            .unwrap()
            .join("model");
        let semantic = Semantic::initialize(&*model_dir, "http://localhost:6334", Arc::new(Configuration::default())).await.unwrap();

        let result = semantic.embed("blog");
        println!("{:?}", result.unwrap());
    }
}