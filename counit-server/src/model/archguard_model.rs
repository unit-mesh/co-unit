use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerService {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) demands: Vec<ContainerDemand>,
    #[serde(default)]
    pub(crate) resources: Vec<ContainerSupply>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerSupply {
    pub(crate) source_url: String,
    pub(crate) source_http_method: String,
    pub(crate) package_name: String,
    pub(crate) class_name: String,
    pub(crate) method_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDemand {
    pub(crate) source_caller: String,
    pub(crate) call_routes: Vec<String>,
    pub(crate) base: String,
    pub(crate) target_url: String,
    pub(crate) target_http_method: String,
    pub(crate) call_data: String,
}
