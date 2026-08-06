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

    pub fn generate_image_url(bucket_name: &str) -> String {
        let supabase = SupabaseConfig::new();
        format!(
            "{}/object/public/{}/{}",
            supabase.storage_base_url,
            bucket_name,
            CommonUtility::generate_unique_name()
        )
    }
}
