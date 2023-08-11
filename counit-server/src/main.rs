use std::net::SocketAddr;

use axum::{Extension, response::IntoResponse, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::server::embedding_api;

pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let bind = SocketAddr::new("0.0.0.0".parse().unwrap(), "8765".parse().unwrap());

    let mut api = Router::new()
        .route("/", get(root))
        .route("/embedding/rest_api", post(embedding_api::create_rest_api_embedding));

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
