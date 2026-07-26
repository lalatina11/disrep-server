use crate::config::server_config::ServerConfig;

pub mod env_config;
pub mod server_config;
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let server = ServerConfig::new();
        AppConfig { server }
    }
}
