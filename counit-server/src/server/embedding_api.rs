use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn create_rest_api_embedding(
    Json(payload): Json<ResetApiRequest>,
) -> (StatusCode, Json<RestApi>) {
    let api: RestApi = RestApi {
        id: 1
    };

    (StatusCode::CREATED, Json(api))
}

#[derive(Deserialize)]
pub struct ResetApiRequest {
    username: String,
}

#[derive(Serialize)]
pub struct RestApi {
    id: u64,
}

#[derive(Debug)]
pub struct RestApiOperation {
    // default: 200
    code: i32,
    http_method: String,
    // http uri path
    path: String,
    // aka, description
    description: String,
    // canonicalName, packageName + "." + className + ":" + methodNam
    canonical_name: String,

    request: Option<String>,
    request_headers: Vec<String>,

    // should be canonicalName = packageName + "." + className, canonicalName
    response: Option<String>,
    response_headers: Vec<String>,
    // "List", "Set" or "Map"
    // response_container: Option<String>,
    // // response type: application/json, application/xml
    // produces: Vec<String>,
    // // request type: application/json, application/xml
    // consumes: String,

    // default to https/http
    protocols: String,
    nickname: Option<String>,
    notes: Option<String>,
    tags: Vec<String>,
    authorizations: Vec<String>,
    extensions: Vec<String>
}