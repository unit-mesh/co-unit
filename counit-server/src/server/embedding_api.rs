use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn create_rest_api_embedding(
    Json(payload): Json<ResetApiRequest>,
) -> (StatusCode, Json<RestApi>) {
    let api: RestApi = RestApi {
        id: 1
    };

    (StatusCode::CREATED, Json(api))
}

#[derive(Deserialize)]
pub struct ResetApiRequest {
    username: String,
}

#[derive(Serialize)]
pub struct RestApi {
    id: u64,
}