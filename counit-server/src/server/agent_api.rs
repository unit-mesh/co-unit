use axum::{body::HttpBody, extract::Query, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::agent::Tool;

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/prompt-generator", post(prompt_generator))
        .route("/functions", get(functions))
}

#[derive(Debug, Deserialize)]
pub struct PromptQuery {
    pub q: String,
}

pub(crate) async fn prompt_generator(
    Query(args): Query<PromptQuery>,
) -> (StatusCode, Json<PromptResult>) {
    let output = PromptResult {
        prompt: "Hello".to_string(),
    };

    (StatusCode::CREATED, Json(output))
}

impl crate::server::ApiResponse for PromptResult {}

#[derive(Serialize)]
pub struct PromptResult {
    pub prompt: String,
}

pub(crate) async fn functions() -> (StatusCode, Json<Vec<Tool>>) {
    (StatusCode::CREATED, Json(vec![]))
}