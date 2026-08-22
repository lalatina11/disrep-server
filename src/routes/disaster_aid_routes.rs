use axum::{Router, middleware::from_fn, routing::post};

use crate::{
    handlers::disaster_aid_handler::DisasterAidHandler,
    middleware::{admin_middleware::AdminMiddleware, auth_middleware::AuthMiddleware},
};

pub struct DiasasterAidRoutes;

impl DiasasterAidRoutes {
    fn protected() -> Router {
        Router::new()
            .merge(Self::admin_authority())
            .layer(from_fn(AuthMiddleware::handle))
    }

    fn admin_authority() -> Router {
        Router::new()
            .route("/", post(DisasterAidHandler::create))
            .layer(from_fn(AdminMiddleware::handle))
    }

    pub fn setup() -> Router {
        Router::new().merge(Self::protected())
    }
}
