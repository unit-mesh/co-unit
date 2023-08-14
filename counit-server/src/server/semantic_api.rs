use axum::Extension;
use axum::body::HttpBody;
use axum::extract::Query;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::application::Application;
use crate::repository::payload::CodePayload;
use crate::repository::semantic_query::SemanticQuery;
use crate::server::{Error, json};

pub(crate) async fn query(
    Query(args): Query<ApiQuery>,
    Extension(app): Extension<Application>,
) -> impl IntoResponse {
    let q = SemanticQuery::from_str(args.q, args.repo_ref);

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

#[derive(Debug, Deserialize)]
pub struct ApiQuery {
    pub q: String,
    pub repo_ref: String,
}

impl crate::server::ApiResponse for QueryResponse {}

#[derive(Serialize)]
pub struct QueryResponse {
    pub data: Vec<CodePayload>,
}
