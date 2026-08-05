use crate::config::env_config::EnvConfig;

pub struct UtilityConfig {
    pub avatar_generator_base_url: String,
}

impl UtilityConfig {
    pub fn new() -> Self {
        let avatar_generator_base_url = EnvConfig::get("AVATAR_GENERATOR_BASE_URL");
        Self {
            avatar_generator_base_url,
        }
    }
}
