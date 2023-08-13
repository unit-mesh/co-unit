use std::sync::Arc;
use anyhow::{bail, Result};
use tracing::warn;
use crate::configuration::Configuration;
use crate::repository::semantic::Semantic;

#[derive(Clone)]
pub struct Application {
    /// User-provided configuration
    pub config: Arc<Configuration>,

    /// Semantic search subsystem
    pub(crate) semantic: Option<Semantic>,
}

impl Application {
    pub async fn initialize(mut config: Configuration) -> Result<Application> {
        let config = Arc::new(config);

        let semantic = match config.qdrant_url {
            Some(ref url) => {
                match Semantic::initialize(&config.model_dir, url, Arc::clone(&config)).await {
                    Ok(semantic) => Some(semantic),
                    Err(e) => {
                        bail!("Qdrant initialization failed: {}", e);
                    }
                }
            }
            None => {
                warn!("Semantic search disabled because `qdrant_url` is not provided. Starting without.");
                None
            }
        };

        Ok(Application {
            config,
            semantic,
        })
    }
}