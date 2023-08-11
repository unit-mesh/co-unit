use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn create_domain_language(
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

