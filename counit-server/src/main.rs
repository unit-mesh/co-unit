use std::net::SocketAddr;

use axum::{Extension, http::StatusCode, Json, Router, routing::post};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let bind = SocketAddr::new("0.0.0.0".parse().unwrap(), "8765".parse().unwrap());

    let mut api = Router::new()
        .route("/", get(root))
        .route("/embedding/rest_api", post(create_rest_api_embedding));

    api = api.route("/health", get(health));

    let mut router = Router::new().nest("/api", api);

    info!(%bind, "starting webserver");
    axum::Server::bind(&bind)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}


async fn health(Extension(app): Extension<String>) {
    println!("health: {}", app);
}


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_rest_api_embedding(
    Json(payload): Json<ResetApiRequest>,
) -> (StatusCode, Json<RestApi>) {
    let api: RestApi = RestApi {
        id: 1
    };

    (StatusCode::CREATED, Json(api))
}

#[derive(Deserialize)]
struct ResetApiRequest {
    username: String,
}

#[derive(Serialize)]
struct RestApi {
    id: u64,
}