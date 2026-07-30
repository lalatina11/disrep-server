use axum::{Router, routing::get};

use crate::handlers::root_handler::RootHandler;

pub struct RootRoutes;

impl RootRoutes {
    pub fn setup() -> Router {
        Router::new().route("/", get(RootHandler::index))
    }
}
