use axum::{Json, Router};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/", post(init_datastructure))
}


pub async fn init_datastructure(
    Json(payload): Json<DataStructure>,
) -> (StatusCode, Json<Response>) {
    let api: Response = Response {
        id: 1
    };

    (StatusCode::CREATED, Json(api))
}

#[derive(Serialize)]
pub struct DataStructure {

}

#[derive(Deserialize)]
pub struct Response {
    id: u64
}