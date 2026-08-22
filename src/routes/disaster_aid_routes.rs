use axum::{Router, routing::post};

use crate::handlers::disaster_aid_handler::DisasterAidHandler;

pub struct DiasasterAidRoutes;

impl DiasasterAidRoutes {
    pub fn setup() -> Router {
        Router::new().route("/", post(DisasterAidHandler::create))
    }
}
