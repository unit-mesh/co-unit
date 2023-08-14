use std::any::Any;
use std::collections::BTreeMap;
use axum::{Extension, Json, Router};
use axum::extract::Query;
use serde::Deserialize;
use utoipa::openapi::path::Operation;
use crate::application::Application;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/v3", post(save_openapi))
}

pub async fn save_openapi(
    Extension(app): Extension<Application>,
    Query(params): Query<OpenApiParams>,
    Json(payload): Json<utoipa::openapi::OpenApi>,
) -> String {
    println!("params: {:?}", &params);
    println!("payload: {:?}", serde_json::to_value(&payload).unwrap());
    "hello".to_string()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiParams {
    repo_ref: String,
    repo_url: String,
    language: String,
}

/// output examples:
/// ```http request
/// ### Send POST request with json body
/// POST http://127.0.0.1:6333/collections/documents/points/search
/// Content-Type: application/json
/// { should return the json request }
///
/// { should return the json response }
/// ```
pub fn format_openapi(openapi: &utoipa::openapi::OpenApi) -> String {
    let mut formatted = String::new();

    for (path, path_item) in &openapi.paths.paths {
        // summary
        if let Some(summary) = &path_item.summary {
            formatted.push_str(&*("### ".to_owned() + summary + "\n"));
        }
        for (item_type, operation) in &path_item.operations {
            println!("item_type: {:?}", serde_json::to_value(&item_type).unwrap());
            println!("operation: {:?}", serde_json::to_value(&operation).unwrap());

        }
    }

    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_openapi() {
        // read from cargo manifest / fixtures/ petstore.json
        let content = std::fs::read_to_string("fixtures/petstore.json").unwrap();
        let openapi = serde_json::from_str::<utoipa::openapi::OpenApi>(&content).unwrap();
        let formatted = format_openapi(&openapi);
        println!("formatted: {}", formatted);
    }
}