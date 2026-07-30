use axum::{Router, routing::get};

use crate::{handlers::root_handler::RootHandler, routes::auth_routes::AuthRoutes};

pub struct ApiRoutes;

impl ApiRoutes {
    pub fn setup() -> Router {
        Router::new()
            .route("/", get(RootHandler::index))
            .nest("/auth", AuthRoutes::setup())
    }
}
