use crate::config::env_config::EnvConfig;

pub struct SupabaseConfig {
    pub project_url: String,
    pub publishable_key: String,
    pub storage_base_url: String, // pub secret_key:String,
    pub admin_token: String,      // pub secret_key:String,
}

impl SupabaseConfig {
    pub fn new() -> Self {
        Self {
            project_url: EnvConfig::get("SUPABASE_PROJECT_URL"),
            publishable_key: EnvConfig::get("SUPABASE_PUBLISHABLE_KEY"),
            storage_base_url: EnvConfig::get("SUPABASE_STORAGE_BASE_URL"),
            admin_token: EnvConfig::get("SUPABASE_ADMIN_TOKEN"),
        }
    }
}
