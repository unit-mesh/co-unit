use axum::{Extension, Json, Router};
use okapi::openapi3::OpenApi;
use crate::application::Application;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/v3", post(save_openapi))
}

pub async fn save_openapi(
    Extension(app): Extension<Application>,
    Json(payload): Json<OpenApi>,
) -> String {
    println!("payload: {:?}", serde_json::to_value(&payload).unwrap());
    "hello".to_string()
}