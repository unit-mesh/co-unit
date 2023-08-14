use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiCollection {
    name: String,
    description: String,
    items: Vec<ApiItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiItem {
    path: String,
    #[serde(default)]
    method: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    operationId: String,
    #[serde(default)]
    tags: Vec<String>,
    request: Option<Request>,
    #[serde(default)]
    response: Vec<Response>,
    #[serde(default)]
    display_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(default)]
    name: String,
    #[serde(default)]
    typ: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BodyMode {
    RAW_TEXT,
    TYPED,
}

impl Default for BodyMode {
    fn default() -> Self {
        BodyMode::TYPED
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(default)]
    parameters: Vec<Parameter>,
    #[serde(default)]
    body: Vec<Parameter>,
    #[serde(default)]
    body_mode: BodyMode,
    #[serde(default)]
    body_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    status: i32,
    #[serde(default)]
    parameters: Vec<Parameter>,
    #[serde(default)]
    body_mode: BodyMode,
    #[serde(default)]
    body_string: String,
}

