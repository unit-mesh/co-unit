use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use serde::Deserialize;

use crate::server::chapi_model::CodeDataStruct;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/:systemId/reporting/class-items", post(save_class_items))
        .route("/:systemId/reporting/container-services", post(save_container))
}

#[derive(Deserialize)]
pub struct ArchGuardParams {
    language: String,
    path: String,
}


pub async fn save_class_items(
    Path(systemId): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<CodeDataStruct>>,
) -> (StatusCode, Json<()>) {
    println!("systemId: {}", systemId);
    println!("inputs: {}", serde_json::to_value(&payload).unwrap());
    println!("save_class_items");

    (StatusCode::CREATED, Json(()))
}

pub async fn save_container(
    Path(systemId): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<ContainerServiceDto>>,
) -> (StatusCode, Json<()>) {
    println!("save_container");

    (StatusCode::CREATED, Json(()))
}
#[derive(Deserialize)]
pub struct ContainerServiceDto {
    name: String,
    demands: Vec<ContainerDemand>,
    resources: Vec<ContainerSupply>,
}

#[derive(Deserialize)]
pub struct ContainerSupply {
    source_url: String,
    source_http_method: String,
    package_name: String,
    class_name: String,
    method_name: String,
}

#[derive(Deserialize)]
pub struct ContainerDemand {
    source_caller: String,
    call_routes: Vec<String>,
    base: String,
    target_url: String,
    target_http_method: String,
    call_data: String,
}
