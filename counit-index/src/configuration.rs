use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
// #[clap(author, version, about, long_about = None)]
pub struct Configuration {
    //
    // Core configuration options
    //
    // #[clap(short, long)]
    #[serde(skip)]
    /// If a config file is given, it will override _all_ command line parameters!
    pub config_file: Option<PathBuf>,

    // #[clap(long)]
    /// Path to dynamic libraries used in the app.
    pub dylib_dir: Option<PathBuf>,

    //
    // Semantic values
    //
    // #[clap(long)]
    /// URL for the qdrant server
    pub qdrant_url: Option<String>,

    // #[clap(long, default_value_os_t = default_model_dir())]
    #[serde(default = "default_model_dir")]
    /// Path to the embedding model directory
    pub model_dir: PathBuf,

    // #[clap(long, default_value_t = default_max_chunk_tokens())]
    #[serde(default = "default_max_chunk_tokens")]
    /// Maximum number of tokens in a chunk (should be the model's input size)
    pub max_chunk_tokens: usize,
}

fn default_model_dir() -> PathBuf {
    "model".into()
}

fn default_max_chunk_tokens() -> usize {
    256
}