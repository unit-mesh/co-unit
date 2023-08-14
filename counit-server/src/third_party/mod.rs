use axum::Router;
pub mod swagger;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .nest("/swagger", swagger::router())
}