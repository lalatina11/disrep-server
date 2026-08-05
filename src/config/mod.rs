use crate::config::{
    database_config::DatabaseConfig, server_config::ServerConfig, supabase_config::SupabaseConfig,
};

pub mod database_config;
pub mod env_config;
pub mod server_config;
pub mod supabase_config;
pub mod utility_config;

pub struct AppConfig {
    pub server: ServerConfig,
    pub supabase: SupabaseConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let server = ServerConfig::new();
        let supabase = SupabaseConfig::new();
        let database = DatabaseConfig::new();
        AppConfig {
            server,
            supabase,
            database,
        }
    }
}
