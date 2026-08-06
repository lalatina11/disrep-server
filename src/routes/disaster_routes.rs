use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{
    handlers::disaster_handler::DisasterHandler, middleware::auth_middleware::AuthMiddleware,
};

pub struct DisasterRoutes;

impl DisasterRoutes {
    pub fn setup() -> Router {
        Router::new()
            .route("/", get(DisasterHandler::get_all))
            .merge(DisasterRoutes::protected())
    }

    fn protected() -> Router {
        Router::new()
            .route("/", post(DisasterHandler::create))
            .route("/upload", post(DisasterHandler::supabase_upload))
            .layer(from_fn(AuthMiddleware::handle))
    }
}
