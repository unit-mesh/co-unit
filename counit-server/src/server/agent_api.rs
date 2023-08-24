use axum::{body::HttpBody, extract::Query, Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::agent::prompts::{hypothetical_document_prompt, tool_prompt};
use crate::agent::tools::{Tool, tools_list};

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/prompt/dsl/generator", get(dsl_generator))
        .route("/prompt/hypo-doc", get(hypothetical_doc))
        .route("/prompt/functions/matching", post(tool_prompter))
        .route("/prompt/functions/list", get(functions))
}

#[derive(Debug, Deserialize)]
pub struct PromptQuery {
    pub q: String,
}

#[derive(Debug, Deserialize)]
pub struct HypoDocQuery {
    pub q: String,
}

pub(crate) async fn hypothetical_doc(
    Query(args): Query<PromptQuery>,
) -> (StatusCode, Json<PromptResult>) {
    let output = PromptResult {
        prompt: hypothetical_document_prompt(&args.q),
    };

    (StatusCode::OK, Json(output))
}


#[derive(Debug, Deserialize)]
pub struct PathListArgs {
    pub paths: Vec<String>,
}

pub(crate) async fn dsl_generator(
    Query(args): Query<PathListArgs>,
) -> (StatusCode, Json<PromptResult>) {
    let output = PromptResult {
        prompt: tool_prompt(&args.paths),
    };

    (StatusCode::OK, Json(output))
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

#[derive(Serialize)]
pub struct PromptResult {
    pub prompt: String,
}

pub(crate) async fn functions() -> (StatusCode, Json<Vec<Tool>>) {
    (StatusCode::OK, Json(Vec::from(tools_list())))
}