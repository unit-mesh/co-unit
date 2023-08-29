use axum::{Json, Router};
use serde::Serialize;

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/", get(list))
}


pub async fn list() -> Json<Vec<LanguageResponse>> {
    Json(vec![
        LanguageResponse {
            id: 1
        },
        LanguageResponse {
            id: 2
        },
    ])
}

#[derive(Serialize)]
pub struct LanguageResponse {
    id: u64
}

