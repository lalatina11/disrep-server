use axum::Router;

use crate::routes::root::RootRoutes;

mod root;
pub struct AppRoutes;

impl AppRoutes {
    pub fn new() -> Router {
        Router::new().merge(RootRoutes::setup())
    }
}
