use axum::{Extension, Json};
use axum::body::HttpBody;
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::application::Application;
use crate::repository::payload::CodePayload;
use crate::repository::semantic_query::SemanticQuery;

pub(crate) async fn query(
    Query(args): Query<ApiQuery>,
    Extension(app): Extension<Application>,
) -> (StatusCode, Json<()>) {
    let q = SemanticQuery::from_str(args.q, "archguard".to_string());
    let results = app.semantic
        .unwrap()
        .search(
            &q,
            10,
            0,
            0.0,
            false,
        )
        .await.unwrap();

    println!("results: {:?}", results);
        (StatusCode::OK, Json(()))
}

#[derive(Debug, Deserialize)]
pub struct ApiQuery {
    pub q: String
}

impl crate::server::ApiResponse for QueryResponse {}

#[derive(Serialize)]
pub struct QueryResponse {
    pub data: Vec<CodePayload>,
}
