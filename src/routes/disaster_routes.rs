use axum::{Router, routing::get};

use crate::handlers::disaster_handler::DisasterHandler;

pub struct DisasterRoutes;

impl DisasterRoutes {
    pub fn setup() -> Router {
        Router::new().route("/", get(DisasterHandler::get_all))
    }
}
