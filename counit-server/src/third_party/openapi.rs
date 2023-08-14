use std::any::Any;
use std::collections::BTreeMap;

use axum::{Extension, Json, Router};
use axum::extract::Query;
use serde::Deserialize;
use utoipa::openapi::{PathItemType, RefOr, Schema};

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

    // let ref_to_ref_schema: Vec<RefOr<Schema>> = Vec::new();
    let mut schema_map: BTreeMap<String, Schema> = BTreeMap::new();

    if let Some(components) = &openapi.components {
        components.schemas.iter().map(|(key, value)| {
            match value {
                RefOr::Ref(ref_obj) => {
                    println!("reference: {:?}", serde_json::to_value(&ref_obj).unwrap());
                }
                RefOr::T(schema) => {
                    match schema {
                        Schema::Array(array) => {
                            schema_map.insert("#/components/schema/".to_owned() + &key.clone(), schema.clone());
                        }
                        Schema::Object(obj) => {
                            schema_map.insert("#/components/schema/".to_owned() + &key.clone(), schema.clone());
                        }
                        _ => {
                            println!("TODO: schema: {:?}", serde_json::to_value(&schema).unwrap());
                        }
                    }
                }
            }
        }).collect()
    }

    for (path, path_item) in &openapi.paths.paths {
        for (item_type, operation) in &path_item.operations {
            if let Some(summary) = &operation.summary {
                formatted.push_str(&*("### ".to_owned() + summary + "\n"));
            }
            if let Some(description) = &operation.description {
                formatted.push_str(&*("> ".to_owned() + description + "\n"));
            }

            let output = format!("{} {}\n", item_to_string(item_type), path);
            formatted.push_str(&output);


            let responses = &operation.responses.responses;
            if responses.len() > 0 {
                formatted.push_str("Content-Type: application/json\n");
            }
        }
    }

    formatted
}


fn item_to_string(path_item: &PathItemType) -> String {
    match path_item {
        PathItemType::Get => "GET".to_string(),
        PathItemType::Post => "POST".to_string(),
        PathItemType::Put => "PUT".to_string(),
        PathItemType::Delete => "DELETE".to_string(),
        PathItemType::Options => "OPTIONS".to_string(),
        PathItemType::Head => "HEAD".to_string(),
        PathItemType::Patch => "PATCH".to_string(),
        PathItemType::Trace => "TRACE".to_string(),
        PathItemType::Connect => "CONNECT".to_string(),
    }
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