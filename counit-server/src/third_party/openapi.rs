use axum::{Extension, Json, Router};
use axum::extract::Query;
use okapi::openapi3::{OpenApi, Operation, RefOr, RequestBody};
use serde::Deserialize;
use crate::application::Application;

pub fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/v3", post(save_openapi))
}

pub async fn save_openapi(
    Extension(app): Extension<Application>,
    Query(params): Query<OpenApiParams>,
    Json(payload): Json<OpenApi>,
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
/// { some json body }
///
/// { some json response }
/// ```
pub fn format_openapi(openapi: &OpenApi) -> String {
    let mut formatted = String::new();

    for (path, path_item) in &openapi.paths {
        let methods_with_operations: Vec<(&str, &Option<Operation>)> = vec![
            ("GET", &path_item.get),
            ("PUT", &path_item.put),
            ("POST", &path_item.post),
            ("DELETE", &path_item.delete),
            ("OPTIONS", &path_item.options),
            ("HEAD", &path_item.head),
            ("PATCH", &path_item.patch),
            ("TRACE", &path_item.trace),
        ];

        for (method, operation_option) in methods_with_operations {
            if let Some(operation) = operation_option {
                formatted.push_str("```http request\n");
                formatted.push_str(&format!("### {} {}\n", method, path));
                formatted.push_str(&format!("{} {}\n", method, path));

                if let Some(request_body) = &operation.request_body {
                    formatted.push_str("Content-Type: application/json\n");

                    match &Some(request_body) {
                        Some(body) => {
                            match body {
                                RefOr::Ref(refItem) => {
                                    formatted.push_str(&format!("{:?}\n", refItem));
                                }
                                RefOr::Object(obj) => {
                                    formatted.push_str(&format!("{:?}\n", obj));
                                }
                            }
                        }
                        None => {}
                    }
                }

                formatted.push_str("```\n");
            }
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
        let openapi = serde_json::from_str::<OpenApi>(&content).unwrap();
        let formatted = format_openapi(&openapi);
        println!("formatted: {}", formatted);
    }
}