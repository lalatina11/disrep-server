use axum::{
    Router,
    middleware::from_fn,
    routing::{get, post},
};

use crate::{handlers::auth_handler::AuthHandler, middleware::auth_middleware::AuthMiddleware};

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn setup() -> Router {
        Router::new()
            .route("/sign-up", post(AuthHandler::sign_up))
            .route("/sign-in", post(AuthHandler::sign_in))
            .merge(ProtectedAuthRoutes::setup())
    }
}

struct ProtectedAuthRoutes;

impl ProtectedAuthRoutes {
    fn setup() -> Router {
        Router::new()
            .route("/me", get(AuthHandler::get_user))
            .layer(from_fn(AuthMiddleware::handle))
    }
}
