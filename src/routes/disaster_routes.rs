use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, patch, post},
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
            .route(
                "/{id}",
                get(DisasterHandler::get_by_id).layer(from_fn(AuthMiddleware::optional)),
            )
            .merge(Self::protected())
    }

    fn protected() -> Router {
        Router::new()
            .route("/", post(DisasterHandler::create))
            .merge(Self::admin_authority())
            .layer(from_fn(AuthMiddleware::handle))
    }

    fn admin_authority() -> Router {
        Router::new()
            .route("/{id}", delete(DisasterHandler::delete))
            .route("/{id}/status", patch(DisasterHandler::update_status))
            .layer(from_fn(AdminMiddleware::handle))
    }
}
