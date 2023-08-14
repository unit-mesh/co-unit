use axum::{Extension, Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use serde::Deserialize;
use tokio::runtime::Handle;

use crate::application::Application;
use crate::model::{CodeDatabaseRelation, CodeDataStruct};
use crate::model::ContainerService;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/:systemId/reporting/class-items", post(save_class_items))
        .route("/:systemId/reporting/container-services", post(save_container))
        .route("/:systemId/reporting/datamap-relations", post(save_datamap))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchGuardParams {
    language: String,
    path: String,
    // same to repo url
    repo_id: String,
}

pub async fn save_datamap(
    Extension(app): Extension<Application>,
    Path(systemId): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<CodeDatabaseRelation>>,
) -> (StatusCode, Json<()>) {

    (StatusCode::CREATED, Json(()))
}

pub async fn save_class_items(
    Extension(app): Extension<Application>,
    Path(systemId): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<CodeDataStruct>>,
) -> (StatusCode, Json<()>) {

    // todo: handle by ourself to convert Http API

    (StatusCode::CREATED, Json(()))
}

pub async fn save_container(
    Extension(app): Extension<Application>,
    Path(systemId): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<ContainerService>>,
) -> (StatusCode, Json<()>) {
    println!("payload: {:?}", serde_json::to_value(&payload).unwrap());
    match app.semantic {
        Some(ref semantic) => {
            payload.iter().for_each(|container| {
                let _ = &container.resources.iter().for_each(|resource| {
                    tokio::task::block_in_place(|| {
                        Handle::current().block_on(async {
                            println!("resource: {:?}", resource.display());
                            semantic.insert_points_for_buffer(
                                params.repo_id.as_str(),
                                params.path.as_str(),
                                resource.display().as_str(),
                            ).await;
                        });
                    });
                });
            });
        }
        None => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(()));
        }
    }

    (StatusCode::CREATED, Json(()))
}
