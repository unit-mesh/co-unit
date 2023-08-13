use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use serde::Deserialize;

use crate::model::CodeDataStruct;
use crate::model::ContainerServiceDto;

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
