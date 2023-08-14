use axum::Router;
pub mod openapi;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .nest("/openapi", openapi::router())
}