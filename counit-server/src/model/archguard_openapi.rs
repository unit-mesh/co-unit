use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiCollection {
    pub name: String,
    pub description: String,
    pub items: Vec<ApiItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiItem {
    pub path: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub operation_id: String,
    #[serde(default)]
    tags: Vec<String>,
    pub request: Option<Request>,
    #[serde(default)]
    pub response: Vec<Response>,
    #[serde(default)]
    pub display_text: String,
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
    RawText,
    Typed,
}

impl Default for BodyMode {
    fn default() -> Self {
        BodyMode::Typed
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Response {
    status: i32,
    #[serde(default)]
    parameters: Vec<Parameter>,
    #[serde(default)]
    body_mode: BodyMode,
    #[serde(default)]
    body_string: String,
}

