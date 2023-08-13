use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerServiceDto {
    #[serde(default)]
    name: String,
    #[serde(default)]
    demands: Vec<ContainerDemand>,
    #[serde(default)]
    resources: Vec<ContainerSupply>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerSupply {
    source_url: String,
    source_http_method: String,
    package_name: String,
    class_name: String,
    method_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDemand {
    source_caller: String,
    call_routes: Vec<String>,
    base: String,
    target_url: String,
    target_http_method: String,
    call_data: String,
}
