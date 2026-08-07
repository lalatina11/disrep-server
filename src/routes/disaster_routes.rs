use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::from_fn,
    routing::{get, patch, post},
};

use crate::{
    handlers::disaster_handler::DisasterHandler,
    middleware::{admin_middleware::AdminMiddleware, auth_middleware::AuthMiddleware},
};

pub struct DisasterRoutes;

impl DisasterRoutes {
    pub fn setup() -> Router {
        Router::new()
            .route("/", get(DisasterHandler::get_all))
            .route("/{id}", get(DisasterHandler::get_by_id))
            .merge(Self::protected())
    }

    fn protected() -> Router {
        Router::new()
            .route("/", post(DisasterHandler::create))
            .merge(Self::admin_authority())
            .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
            .layer(from_fn(AuthMiddleware::handle))
    }

    fn admin_authority() -> Router {
        Router::new()
            .route("/{id}/approve", patch(DisasterHandler::approve))
            .layer(from_fn(AdminMiddleware::handle))
    }
}
