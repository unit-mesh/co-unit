use axum::{body::HttpBody, extract::Query, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::agent::prompts::tool_prompt;
use crate::dsl::query_description::QAExample;

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/prompt/explain", get(explain_query))

        .route("/prompt/functions/matching", post(tool_prompter))
}

#[derive(Debug, Deserialize)]
pub struct PromptQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct PromptResult {
    pub prompt: String,
}

pub(crate) async fn explain_query(
    Query(args): Query<PromptQuery>,
) -> (StatusCode, Json<PromptResult>) {
    let output = PromptResult {
        prompt: QAExample::prompt(&args.q),
    };

    (StatusCode::OK, Json(output))
}

#[derive(Debug, Deserialize)]
pub struct PathListArgs {
    pub paths: Vec<String>,
}

pub(crate) async fn tool_prompter(
    Query(args): Query<PromptQuery>,
) -> (StatusCode, Json<PromptResult>) {
    let paths = vec![args.q];
    let output = PromptResult {
        prompt: tool_prompt(&paths),
    };

    (StatusCode::OK, Json(output))
}

impl crate::server::ApiResponse for PromptResult {}
