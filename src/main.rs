use core::panic;

use crate::{config::AppConfig, routes::AppRoutes};

pub mod config;
pub mod handlers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    let config = AppConfig::new();
    let app = AppRoutes::new();
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to create tokio listener"));
    println!(
        "Server Listening on http://{}:{}",
        config.server.host, config.server.port
    );
    axum::serve::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Failed to run the server"));
}
