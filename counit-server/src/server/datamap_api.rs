use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn datamap_embedding(
    Json(payload): Json<DatamapRequest>,
) -> (StatusCode, Json<DatamapEmbedding>) {
    let api: DatamapEmbedding = DatamapEmbedding {
        id: 1
    };

    (StatusCode::CREATED, Json(api))
}

#[derive(Deserialize)]
pub struct DatamapRequest {}

#[derive(Serialize)]
pub struct DatamapEmbedding {
    id: u64,
}

#[derive(Serialize)]
pub struct CodeDatabaseRelation {
    package_name: String,
    class_name: String,
    function_name: String,
    tables: Vec<String>,
    sqls: Vec<String>,
}