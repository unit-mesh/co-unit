use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode, Json, Router,
};
use serde::Deserialize;
use tokio::runtime::Handle;

use crate::application::Application;
use crate::model::{
    archguard_openapi::ApiCollection,
    CodeDatabaseRelation, CodeDataStruct, ContainerService,
};
use crate::repository::payload::PayloadType;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/:systemId/reporting/class-items", post(save_class_items))
        .route("/:systemId/reporting/container-services", post(save_container))
        .route("/:systemId/reporting/datamap-relations", post(save_datamap))
        .route("/:systemId/reporting/openapi", post(save_openapi))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArchGuardParams {
    language: String,
    path: String,
    repo_id: String,
    // repo_ref: String,
}

pub async fn save_openapi(
    Extension(app): Extension<Application>,
    Path(system_id): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<ApiCollection>>,
) -> (StatusCode, Json<()>) {
    let repo_ref = params.repo_id.clone();

    match app.semantic {
        Some(ref semantic) => {
            payload.iter().for_each(|collection| {
                let _ = &collection.items.iter().for_each(|item| {
                    tokio::task::block_in_place(|| {
                        Handle::current().block_on(async {
                            println!("display_text {:?}", &item.display_text);
                            let _ = semantic.insert_points_for_buffer(
                                params.repo_id.as_str(),
                                repo_ref.as_str(),
                                params.path.as_str(),
                                item.display_text.as_str(),
                                params.language.as_str(),
                                PayloadType::OpenApi,
                                item.display_text.as_str(),
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


pub async fn save_datamap(
    Extension(app): Extension<Application>,
    Path(system_id): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<CodeDatabaseRelation>>,
) -> (StatusCode, Json<()>) {
    let repo_ref = params.repo_id.clone();
    println!("save_datamap {:?}", repo_ref);

    match app.semantic {
        Some(ref semantic) => {
            payload.iter().for_each(|relation| {
                tokio::task::block_in_place(|| {
                    Handle::current().block_on(async {
                        let display_text = &relation.to_string();

                        println!("display_text {:?}", &display_text);
                        let _ = semantic.insert_points_for_buffer(
                            params.repo_id.as_str(),
                            repo_ref.as_str(),
                            params.path.as_str(),
                            display_text.as_str(),
                            params.language.as_str(),
                            PayloadType::DatabaseMap,
                            display_text.as_str(),
                        ).await;
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

pub async fn save_class_items(
    Extension(app): Extension<Application>,
    Path(system_id): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<CodeDataStruct>>,
) -> (StatusCode, Json<()>) {
    let repo_ref = params.repo_id.clone();
    println!("save_class_items {:?}", repo_ref);

    match app.semantic {
        Some(ref semantic) => {
            payload.iter().for_each(|class| {
                let _ = &class.functions.iter().for_each(|method| {
                    tokio::task::block_in_place(|| {
                        Handle::current().block_on(async {
                            let display_text = &method.display();
                            let origin_content = &method.content;

                            println!("display_text {:?}", display_text);
                            let _ = semantic.insert_points_for_buffer(
                                params.repo_id.as_str(),
                                repo_ref.as_str(),
                                params.path.as_str(),
                                display_text.as_str(),
                                params.language.as_str(),
                                PayloadType::Code,
                                origin_content,
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

pub async fn save_container(
    Extension(app): Extension<Application>,
    Path(system_id): Path<u32>,
    Query(params): Query<ArchGuardParams>,
    Json(payload): Json<Vec<ContainerService>>,
) -> (StatusCode, Json<()>) {
    let repo_ref = params.repo_id.clone();
    println!("save_container {:?}", repo_ref);

    match app.semantic {
        Some(ref semantic) => {
            payload.iter().for_each(|container| {
                let _ = &container.resources.iter().for_each(|resource| {
                    tokio::task::block_in_place(|| {
                        Handle::current().block_on(async {
                            let display_text = resource.display();
                            println!("resource: {:?}", display_text);
                            let _ = semantic.insert_points_for_buffer(
                                params.repo_id.as_str(),
                                repo_ref.as_str(),
                                params.path.as_str(),
                                display_text.as_str(),
                                params.language.as_str(),
                                PayloadType::HttpApi,
                                display_text.as_str(),
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
