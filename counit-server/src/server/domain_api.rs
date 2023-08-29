use axum::{Extension, Json, Router};
use axum::http::StatusCode;

use crate::application::Application;
use crate::domain::domain_record::DomainRecord;

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/", get(list))
}


pub async fn list(
    Extension(app): Extension<Application>,
) -> (StatusCode, Json<Vec<DomainRecord>>) {
    let records = app.transpiler.domain_records.clone();
    return (StatusCode::OK, Json(records));
}
