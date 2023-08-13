use std::sync::Arc;
use crate::configuration::Configuration;

#[derive(Clone, Debug)]
pub struct Application {
    pub config: Arc<Configuration>,
}

impl Application {
    pub fn initialize(mut config: Configuration) -> Self {
        let config = Arc::new(config);

        Application {
            config,
        }
    }
}