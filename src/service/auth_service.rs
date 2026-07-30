use axum::http::{HeaderMap, header as HeaderType};
use reqwest::Client;
use tracing::error;

use crate::{
    config::supabase_config::SupabaseConfig,
    error::AuthError,
    models::auth_model::{SignInPayload, SignUpPayload},
    utils::responses::auth_responses::{SignUpAndInErrorResponse, SignUpAndInSuccessResponse},
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_up(payload: SignUpPayload) -> Result<SignUpAndInSuccessResponse, AuthError> {
        let supabase_config = SupabaseConfig::new();
        let fetch = Client::new();
        let url = format!("{}/auth/v1/signup", supabase_config.project_url);
        let res = fetch
            .post(url)
            .header("Content-Type", "application/json")
            .header("apikey", supabase_config.publishable_key)
            .json(&payload)
            .send()
            .await
            .map_err(|err| {
                println!("Response error");
                AuthError::internal()
            })?
            .text()
            .await
            .map_err(|err| {
                println!("Parsing text error");
                AuthError::internal()
            })?;

        if let Ok(data) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            return Ok(data);
        }

        if let Ok(err) = serde_json::from_str::<SignUpAndInErrorResponse>(&res) {
            return Err(AuthError {
                message: err.msg,
                status: err.code,
            });
        }

        error!("Unexpected Supabase response: {}", res);
        Err(AuthError::internal())
    }

    pub async fn sign_in(payload: SignInPayload) {}

    pub async fn get_user(headers: &HeaderMap) -> Result<String, AuthError> {
        let token = headers
            .get(HeaderType::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or("".to_string());

        Ok(token)
    }
}
