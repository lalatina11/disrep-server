use crate::config::env_config::EnvConfig;

pub struct SupabaseConfig {
    pub project_url: String,
    pub publishable_key: String,
    // pub secret_key:String,
    // pub jwt_secret_key:String,
}

impl SupabaseConfig {
    pub fn new() -> Self {
        Self {
            project_url: EnvConfig::get("SUPABASE_PROJECT_URL"),
            publishable_key: EnvConfig::get("SUPABASE_PUBLISHABLE_KEY"),
        }
    }
}
