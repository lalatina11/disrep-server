use reqwest::{Client, header as HeaderType};

use crate::{
    config::supabase_config::SupabaseConfig, error::ServiceError, models::auth_model::SignUpPayload,
};

pub struct SupabaseService;

impl SupabaseService {
    pub async fn sign_up_user(payload: SignUpPayload) -> Result<String, ServiceError> {
        let supabase_config = SupabaseConfig::new();
        let fetch = Client::new();
        let url = format!("{}/auth/v1/signup", supabase_config.project_url);
        let res = fetch
            .post(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| {
                println!("Response error");
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                ServiceError::internal()
            })?;
        Ok(res)
    }
}
