use axum::{Router, extract::DefaultBodyLimit, routing::post};

use crate::handlers::upload_handler::UploadHandler;

pub struct UploadRoutes;

impl UploadRoutes {
    pub fn setup() -> Router {
        Router::new()
            .route("/image", post(UploadHandler::image))
            .route("/video", post(UploadHandler::video))
            .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
    }
}
