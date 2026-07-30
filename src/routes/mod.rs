use axum::{Router, middleware::from_fn};

use crate::{
    middleware::{error_middleware::ErrorMiddleware, logger_middleware::LoggerMiddleware},
    routes::{api::ApiRoutes, root::RootRoutes},
};

mod api;
mod auth;
mod root;
pub struct AppRoutes;

impl AppRoutes {
    pub fn new() -> Router {
        Router::new()
            .merge(RootRoutes::setup())
            .nest("/api", ApiRoutes::setup())
            .layer(from_fn(LoggerMiddleware::handler))
            .layer(from_fn(ErrorMiddleware::generic_error))
    }
}
