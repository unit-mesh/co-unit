use std::env;
use std::path::Path;
use std::sync::Arc;
use ndarray::Axis;
use ort::{
    tensor::InputTensor,
    Environment, ExecutionProvider, GraphOptimizationLevel, LoggingLevel, SessionBuilder};
use ort::tensor::{FromArray, OrtOwnedTensor};

use qdrant_client::client::{QdrantClient, QdrantClientConfig};
use qdrant_client::prelude::{CreateCollection, Distance, SearchPoints};
use qdrant_client::qdrant::{CollectionOperationResponse, FieldCondition, FieldType, Filter, Match, ScoredPoint, VectorParams, vectors_config, VectorsConfig, with_payload_selector, with_vectors_selector, WithPayloadSelector, WithVectorsSelector};
use qdrant_client::qdrant::r#match::MatchValue;
use thiserror::Error;

use tracing::{debug, info, trace, warn};

use crate::semantic::configuration::Configuration;
use crate::semantic::semantic_query::SemanticQuery;

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
