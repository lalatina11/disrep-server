use axum::{Router, extract::DefaultBodyLimit, routing::post};

use crate::handlers::upload_handler::UploadHandler;

const ONE_MEGA_BYTES: usize = 1024 * 1024;

pub struct UploadRoutes;

impl UploadRoutes {
    pub fn setup() -> Router {
        Router::new()
            .route("/image", post(UploadHandler::image))
            .route("/video", post(UploadHandler::video))
            .layer(DefaultBodyLimit::max(500 * ONE_MEGA_BYTES)) // 500 MB
    }
}
