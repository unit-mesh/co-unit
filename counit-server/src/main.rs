use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use crate::application::Application;
use crate::configuration::Configuration;

use crate::server::{archguard_api, semantic_api, translate_api};

pub mod server;
pub mod model;
pub mod repository;
pub mod application;
pub mod configuration;
pub mod third_party;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Configuration::default();

    let bind = SocketAddr::new(config.host.parse()?, config.port);
    let app = Application::initialize(config).await?;

    let mut api = Router::new()
        .with_state(app.clone())
        .route("/", get(root))
        .route("/query", get(semantic_api::query))

        // knowledge init
        .nest("/translate/domain_language", translate_api::router())
        //align to archguard api
        .nest("/scanner", archguard_api::router())
        .nest("/index/third-part", third_party::router())
        ;

    api = api.route("/health", get(health));

    let api = api
        .layer(Extension(app.clone()))
        .layer(CorsLayer::permissive())
        .layer(CatchPanicLayer::new());

    let mut router = Router::new().nest("/api", api);

    info!(%bind, "starting webserver");

    axum::Server::bind(&bind)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}


async fn health(Extension(app): Extension<Application>) -> String {
    return serde_json::to_string::<Configuration>(&*app.config.clone()).unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
