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

    #[serde(default = "default_domain_language_dir")]
    /// Path to the domain language directory, supported format: .csv, .json
    pub domain_language_dir: Option<PathBuf>,
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

fn default_domain_language_dir() -> Option<PathBuf> {
    Some("domain".into())
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            host: "0.0.0.0".to_string(),
            port: 8765,
            dylib_dir: None,
            qdrant_url: Some("http://127.0.0.1:6334".into()),
            model_dir: Path::new(env!("CARGO_MANIFEST_DIR")).parent()
                .unwrap()
                .join("model"),
            domain_language_dir: Some(Path::new(env!("CARGO_MANIFEST_DIR")).parent()
                .unwrap()
                .join("domain")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_from_json() {
        let config = r#"{
  "host": "0.0.0.0",
  "port": 8765,
  "qdrant_url": "http://127.0.0.1:6334",
  "model_dir": "public/model",
  "dylib_dir": "public"
}
"#;

        let config: Configuration = serde_json::from_str(config).unwrap();
        assert_eq!(config.host, "0.0.0.0");
    }
}