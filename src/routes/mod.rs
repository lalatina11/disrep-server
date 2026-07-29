use axum::Router;

use crate::routes::{api::ApiRoutes, root::RootRoutes};

mod api;
mod auth;
mod root;
pub struct AppRoutes;

impl AppRoutes {
    pub fn new() -> Router {
        Router::new()
            .merge(RootRoutes::setup())
            .nest("/api", ApiRoutes::setup())
    }
}
