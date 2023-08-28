use std::sync::Arc;
use anyhow::{bail, Result};
use tracing::warn;
use crate::configuration::Configuration;
use crate::domain::domain_transpiler::DomainTranspiler;
use crate::repository::semantic::Semantic;

#[derive(Clone)]
pub struct Application {
    /// User-provided configuration
    pub config: Arc<Configuration>,

    pub transpiler: Arc<DomainTranspiler>,

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

        let mut transpiler: Arc<DomainTranspiler>;
        if config.domain_language_dir.is_some() {
            transpiler = Arc::new(DomainTranspiler::new(config.domain_language_dir.as_ref().unwrap()));
        } else {
            transpiler = Arc::new(DomainTranspiler::empty());
        };

        Ok(Application {
            config,
            transpiler,
            semantic,
        })
    }
}