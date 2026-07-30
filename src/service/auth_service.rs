use axum::http::{HeaderMap, header as HeaderType};
use reqwest::Client;
use tracing::error;

use crate::{
    config::supabase_config::SupabaseConfig,
    error::AuthError,
    models::auth_model::{SignInPayload, SignUpPayload},
    utils::responses::auth_responses::{
        GetUserSuccessResponse, SignUpAndInSuccessResponse, SupabaseAuthErrorResponse,
    },
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_up(payload: SignUpPayload) -> Result<SignUpAndInSuccessResponse, AuthError> {
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
                AuthError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                AuthError::internal()
            })?;

        if let Ok(data) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            return Ok(data);
        }

        if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(AuthError {
                message: err.msg,
                status: err.code,
            });
        }

        error!("Unexpected Supabase response: {}", res);
        Err(AuthError::internal())
    }

    pub async fn sign_in(payload: SignInPayload) -> Result<SignUpAndInSuccessResponse, AuthError> {
        let supabase_config = SupabaseConfig::new();
        let fetch = Client::new();
        let url = format!(
            "{}/auth/v1/token?grant_type=password",
            supabase_config.project_url
        );
        let res = fetch
            .post(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| {
                println!("Response error");
                AuthError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                AuthError::internal()
            })?;

        if let Ok(data) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            return Ok(data);
        }

        if let Ok(data) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(AuthError {
                message: data.msg,
                status: data.code,
            });
        }

        Err(AuthError::internal())
    }

    pub async fn get_user(headers: &HeaderMap) -> Result<GetUserSuccessResponse, AuthError> {
        let token = headers
            .get(HeaderType::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or("".to_string());

        if token == "" {
            return Err(AuthError {
                message: "token are required".to_string(),
                status: 401,
            });
        }

        let supabase_config = SupabaseConfig::new();
        let url = format!("{}/auth/v1/user", supabase_config.project_url);
        let fetch = Client::new();
        let res = fetch
            .get(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .header(HeaderType::AUTHORIZATION, token)
            .send()
            .await
            .map_err(|err| {
                error!("Supabase response error {}", err);
                AuthError::internal()
            })?
            .text()
            .await
            .map_err(|err| {
                error!("Failed to parsing text body {}", err);
                AuthError::internal()
            })?;

        if let Ok(data) = serde_json::from_str::<GetUserSuccessResponse>(&res) {
            return Ok(data);
        }

        if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(AuthError {
                message: err.msg,
                status: err.code,
            });
        }

        Err(AuthError::internal())
    }
}
