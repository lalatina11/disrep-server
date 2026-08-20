use crate::config::env_config::EnvConfig;

pub struct SupabaseConfig {
    pub project_url: String,
    pub publishable_key: String,
    pub storage_base_url: String,
    pub admin_token: String,
    pub storage_bucket_name: String,
}

impl SupabaseConfig {
    pub fn new() -> Self {
        let project_url = EnvConfig::get("SUPABASE_PROJECT_URL");
        Self {
            project_url: project_url.clone(),
            publishable_key: EnvConfig::get("SUPABASE_PUBLISHABLE_KEY"),
            storage_base_url: format!("{}/storage/v1", project_url),
            admin_token: EnvConfig::get("SUPABASE_ADMIN_TOKEN"),
            storage_bucket_name: EnvConfig::get("SUPABASE_STORAGE_BUCKET_NAME"),
        }
    }
}
