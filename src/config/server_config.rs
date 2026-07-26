use crate::config::env_config::EnvConfig;

pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            host: EnvConfig::get("HOST"),
            port: EnvConfig::get("PORT"),
        }
    }
}
