use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::auth_handler::AuthHandler;

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
        Router::new().route("/me", get(AuthHandler::get_user))
    }
}
