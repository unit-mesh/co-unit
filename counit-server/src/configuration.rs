use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    pub dylib_dir: Option<PathBuf>,

    /// URL for the qdrant server
    pub qdrant_url: Option<String>,

    #[serde(default = "default_model_dir")]
    /// Path to the embedding model directory
    pub model_dir: PathBuf,
}

const fn default_port() -> u16 {
    7878
}

fn default_host() -> String {
    String::from("127.0.0.1")
}

fn default_model_dir() -> PathBuf {
    "model".into()
}


impl Configuration {
    pub fn default() -> Self {
        Configuration {
            host: "0.0.0.0".to_string(),
            port: 8765,
            dylib_dir: default_model_dir().into(),
            qdrant_url: Some("http://127.0.0.1:6334".into()),
            model_dir: Path::new(env!("CARGO_MANIFEST_DIR")).parent()
                .unwrap()
                .join("model"),
        }
    }
}