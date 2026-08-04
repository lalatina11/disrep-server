use crate::config::env_config::EnvConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Staging,
    Production,
}

impl AppEnv {
    pub fn new() -> Self {
        let status = EnvConfig::get("ENV");
        if status.to_lowercase() == "production" {
            return AppEnv::Production;
        }
        if status.to_lowercase() == "staging" {
            return AppEnv::Staging;
        }
        AppEnv::Development
    }

    pub fn to_string(self) -> String {
        match self {
            AppEnv::Development => "development".to_string(),
            AppEnv::Staging => "staging".to_string(),
            AppEnv::Production => "production".to_string(),
        }
    }
}

pub struct ServerConfig {
    pub host: String,
    pub port: String,
    pub env: AppEnv,
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            host: EnvConfig::get("HOST"),
            port: EnvConfig::get("PORT"),
            env: AppEnv::new(),
        }
    }
}
