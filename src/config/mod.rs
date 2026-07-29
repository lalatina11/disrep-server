use crate::config::{server_config::ServerConfig, supabase_config::SupabaseConfig};

pub mod env_config;
pub mod server_config;
pub mod supabase_config;

pub struct AppConfig {
    pub server: ServerConfig,
    pub supabase: SupabaseConfig,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let server = ServerConfig::new();
        let supabase = SupabaseConfig::new();
        AppConfig { server, supabase }
    }
}
