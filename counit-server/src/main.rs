use std::net::SocketAddr;

use axum::{Extension, Router, routing::get};
use axum::extract::DefaultBodyLimit;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::application::Application;
use crate::configuration::Configuration;
use crate::server::{agent_api, archguard_api, semantic_api, domain_api};

pub mod server;
pub mod model;
pub mod repository;
pub mod application;
pub mod configuration;
pub mod agent;
pub mod dsl;
pub mod domain;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // load configuration from public/config.json if exists
    let config_file = std::path::Path::new("public/config.json");

    let mut config: Configuration;
    if config_file.exists() {
        config = serde_json::from_str(&std::fs::read_to_string(config_file)?)?;
        info!("Configuration loaded from public/config.json");
    } else {
        // for development only
        config = Configuration::default()
    }

    let bind = SocketAddr::new(config.host.parse()?, config.port);
    let app = Application::initialize(config).await?;

    let mut api = Router::new().with_state(app.clone())
        .route("/", get(root))
        // core api for query
        .route("/query", get(semantic_api::query))
        .route("/text-embedding", get(semantic_api::embedding))

        // the agent api
        .nest("/agent", agent_api::router())

        .nest("/domain", domain_api::router())

        //align to archguard api
        .nest("/scanner", archguard_api::router())
        ;

    api = api.route("/health", get(health));

    let api = api
        .layer(Extension(app.clone()))
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::permissive())
        .layer(CatchPanicLayer::new());

    let mut router = Router::new()
        .nest("/api", api);

    info!(%bind, "starting webserver");


    // axum::Server::bind(&bind)
    //     .serve(router.into_make_service())
    //     .await?;

    let listener = tokio::net::TcpListener::bind(&bind).await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}


async fn health(Extension(app): Extension<Application>) -> String {
    return serde_json::to_string::<Configuration>(&*app.config.clone()).unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
