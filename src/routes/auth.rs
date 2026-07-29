use axum::{Router, routing::post};

use crate::handlers::auth_handler::AuthHandler;

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn setup() -> Router {
        Router::new().route("/sign-up", post(AuthHandler::sign_up))
    }
}
