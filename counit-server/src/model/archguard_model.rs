use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContainerService {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) demands: Vec<ContainerDemand>,
    #[serde(default)]
    pub(crate) resources: Vec<ContainerSupply>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContainerSupply {
    pub(crate) source_url: String,
    pub(crate) source_http_method: String,
    pub(crate) package_name: String,
    pub(crate) class_name: String,
    pub(crate) method_name: String,
}

impl ContainerSupply {
    pub fn display(&self) -> String {
        format!("{}.{}.{} -> {} {}", self.package_name, self.class_name, self.method_name, self.source_http_method, self.source_url)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContainerDemand {
    #[serde(default)]
    /// Path to the embedding model directory
    pub(crate) source_caller: String,
    #[serde(default)]
    pub(crate) call_routes: Vec<String>,
    #[serde(default)]
    pub(crate) base: String,
    #[serde(default)]
    pub(crate) target_url: String,
    #[serde(default)]
    pub(crate) target_http_method: String,
    #[serde(default)]
    pub(crate) call_data: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDatabaseRelation {
    package_name: String,
    class_name: String,
    function_name: String,
    #[serde(default)]
    tables: Vec<String>,
    #[serde(default)]
    sqls: Vec<String>,
    relations: Vec<NodeRelation>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRelation {
    source: String,
    target: String
}

