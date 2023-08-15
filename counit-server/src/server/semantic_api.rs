use axum::{
    body::HttpBody, Extension, extract::Query, response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::application::Application;
use crate::repository::{
    payload::CodePayload, semantic_query::SemanticQuery,
};
use crate::repository::payload::PayloadType;
use crate::repository::semantic::Embedding;
use crate::server::{Error, json};

pub(crate) async fn query(
    Query(args): Query<ApiQuery>,
    Extension(app): Extension<Application>,
) -> impl IntoResponse {
    let q = SemanticQuery::from_str(args.q, args.repo_ref, args.query_type);

    let result = app.semantic
        .unwrap()
        .search(&q, 10, 0, 0.0, false)
        .await;

    match result {
        Ok(vec) => {
            Ok(json(QueryResponse { data: vec }))
        }
        Err(err) => {
            Err(Error::from(err))
        }
    }
}

pub(crate) async fn embedding(
    Query(args): Query<SimpleQuery>,
    Extension(app): Extension<Application>,
) -> impl IntoResponse {
    let result = app.semantic
        .unwrap()
        .embed(&args.q);

    match result {
        Ok(vec) => {
            Ok(json(EmbeddingResponse { data: vec }))
        }
        Err(err) => {
            Err(Error::from(err))
        }
    }
}

impl crate::server::ApiResponse for EmbeddingResponse {}

#[derive(Serialize)]
pub struct EmbeddingResponse {
    pub data: Embedding,
}


#[derive(Debug, Deserialize)]
pub struct SimpleQuery {
    pub q: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiQuery {
    pub q: String,
    pub repo_ref: String,
    pub query_type: PayloadType,
}

impl crate::server::ApiResponse for QueryResponse {}

#[derive(Serialize)]
pub struct QueryResponse {
    pub data: Vec<CodePayload>,
}
