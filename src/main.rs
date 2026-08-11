use core::panic;

use crate::{config::AppConfig, routes::AppRoutes};

pub mod config;
pub mod constants;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod service;
pub mod utils;

#[tokio::main]
async fn main() {
    let config = AppConfig::new();
    let app = AppRoutes::new();

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to create tokio listener"));
    let mode = config.server.env.to_string().to_uppercase();
    println!(
        "\n[{} MODE] | Server Listening on http://{}:{}\n",
        mode, config.server.host, config.server.port
    );
    axum::serve::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Failed to run the server"));
}
