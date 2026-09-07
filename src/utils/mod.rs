use axum::http::{HeaderMap, header as HeaderType};
use chrono::Utc;

use crate::config::supabase_config::SupabaseConfig;

pub mod request;
pub mod responses;

pub struct CommonUtility;

impl CommonUtility {
    pub fn generate_unique_name() -> String {
        let now = Utc::now();
        format!(
            "{}-{}-{}",
            now.date_naive(),
            now.time(),
            uuid::Uuid::new_v4()
        )
    }

    pub fn generate_media_url(media_url: String) -> String {
        let supabase = SupabaseConfig::new();
        if media_url.starts_with("http") {
            return media_url;
        }
        format!("{}/object/public/{}", supabase.storage_base_url, media_url)
    }

    pub fn get_access_token_from_headers(headers: &HeaderMap) -> String {
        headers
            .get(HeaderType::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or("".to_string())
    }
}
