use axum::Router;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/", post(save_openapi))
}

pub async fn save_openapi() -> String {
    "hello".to_string()
}