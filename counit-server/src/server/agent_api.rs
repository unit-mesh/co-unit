use axum::{Json, Router};

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        // .route("/prompt-generator", post(prompt_generator))
}

pub struct PromptResult {
    pub prompt: String,
    pub completion: String,
}

// pub async fn prompt_generator() -> Json<PromptResult> {
//     Json(PromptResult {
//         prompt: "Hello".to_string(),
//         completion: "World".to_string(),
//     })
// }
