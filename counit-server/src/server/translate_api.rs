use axum::http::StatusCode;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/", get(list))
        .route("/", post(init_domain_language))
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

pub async fn init_domain_language(
    Json(payload): Json<LanguageDict>,
) -> (StatusCode, Json<LanguageResponse>) {
    let api: LanguageResponse = LanguageResponse {
        id: 1
    };

    (StatusCode::CREATED, Json(api))
}


#[derive(Deserialize)]
pub struct LanguageDict {}

#[derive(Serialize)]
pub struct LanguageResponse {
    id: u64,
}

