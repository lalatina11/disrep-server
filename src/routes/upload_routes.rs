use axum::{Router, routing::post};

use crate::handlers::upload_handler::UploadHandler;

pub struct UploadRoutes;

impl UploadRoutes {
    pub fn setup() -> Router {
        Router::new().route("/image", post(UploadHandler::image))
    }
}
