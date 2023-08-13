use std::net::SocketAddr;

use axum::{Extension, Router, routing::{get, post}};
use tracing::info;

use crate::server::{archguard_api, datamap_api, embedding_api, translate_api};
pub mod server;
pub mod model;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let bind = SocketAddr::new("0.0.0.0".parse().unwrap(), "8765".parse().unwrap());

    let mut api = Router::new()
        .route("/", get(root))
        .route("/embedding/rest_api", post(embedding_api::rest_api_embedding))
        .route("/embedding/datamap", post(datamap_api::datamap_embedding))

        // knowledge init
        .nest("/translate/domain_language", translate_api::router())
        //align to archguard api
        .nest("/scanner", archguard_api::router())
        ;

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

async fn root() -> &'static str {
    "Hello, World!"
}
