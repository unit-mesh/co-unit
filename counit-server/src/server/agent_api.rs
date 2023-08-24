use axum::{body::HttpBody, extract::Query, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::agent::prompts::tool_prompt;

use crate::agent::tools::{Tool, tools_list};

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/functions/matching", post(prompt_generator))
        .route("/functions/list", get(functions))
}

#[derive(Debug, Deserialize)]
pub struct PromptQuery {
    pub q: String,
}

pub(crate) async fn prompt_generator(
    Query(args): Query<PromptQuery>,
) -> (StatusCode, Json<PromptResult>) {
    let paths = vec![args.q];
    let output = PromptResult {
        prompt: tool_prompt(&paths),
    };

    (StatusCode::CREATED, Json(output))
}

impl crate::server::ApiResponse for PromptResult {}

#[derive(Serialize)]
pub struct PromptResult {
    pub prompt: String,
}

pub(crate) async fn functions() -> (StatusCode, Json<Vec<Tool>>) {
    (StatusCode::CREATED, Json(Vec::from(tools_list())))
}