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
            .nest("/protected", ProtectedAuthRoutes::setup())
    }
}

struct ProtectedAuthRoutes;

impl ProtectedAuthRoutes {
    fn setup() -> Router {
        Router::new().route("/get-user", get(AuthHandler::get_user))
    }
}
