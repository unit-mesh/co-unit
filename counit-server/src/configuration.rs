use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    pub dylib_dir: Option<PathBuf>,
}

const fn default_port() -> u16 {
    7878
}

fn default_host() -> String {
    String::from("127.0.0.1")
}


impl Configuration {
    pub fn default() -> Self {
        Configuration {
            host: "0.0.0.0".to_string(),
            port: 8765,
            dylib_dir: None,
        }
    }

}