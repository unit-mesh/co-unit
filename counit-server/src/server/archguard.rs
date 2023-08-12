use axum::extract::Path;
use axum::Router;

pub(crate) fn router() -> Router {
    use axum::routing::*;

    Router::new()
        .route("/:systemId/reporting/class-items", post(save_class_items))
        .route("/:systemId/reporting/container-services", post(save_container))
}

async fn save_class_items(
    Path(systemId): Path<u32>
) {
    println!("save_class_items");
}

async fn save_container(
    Path(systemId): Path<u32>
) {
    println!("save_container");
}