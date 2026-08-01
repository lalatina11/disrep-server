use axum::http::{HeaderMap, header as HeaderType};
use reqwest::Client;
use tracing::error;

use crate::{
    config::supabase_config::SupabaseConfig,
    error::AuthError,
    models::{
        auth_model::{AuthPayload, SignInPayload, SignUpPayload},
        user_model::UserModel,
    },
    service::user_service::UserService,
    utils::responses::auth_responses::{
        GetUserSuccessResponse, SignUpAndInSuccessResponse, SupabaseAuthErrorResponse,
    },
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_up(payload: SignUpPayload) -> Result<AuthPayload, AuthError> {
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

        if let Ok(is_sign_up_success) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            let user_model_parsing = UserService::create_user(
                is_sign_up_success.user.id,
                is_sign_up_success.user.email,
                is_sign_up_success.user.user_metadata.display_name,
            )
            .await;
            if let Ok(user_model) = user_model_parsing {
                return Ok(user_model.to_payload(is_sign_up_success.access_token));
            }
            return Err(AuthError::internal());
        }

        if let Ok(is_sign_up_err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(AuthError {
                message: is_sign_up_err.msg,
                status: is_sign_up_err.code,
            });
        }

        error!("Unexpected Supabase response: {}", res);
        Err(AuthError::internal())
    }

    pub async fn sign_in(payload: SignInPayload) -> Result<AuthPayload, AuthError> {
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

        if let Ok(is_sign_in_success) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            let user_model_parsing = UserService::get_user_by_id(is_sign_in_success.user.id).await;
            if let Ok(user_model) = user_model_parsing {
                return Ok(user_model.to_payload(is_sign_in_success.access_token));
            }
            return Err(AuthError::internal());
        }

        if let Ok(is_sign_in_err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(AuthError {
                message: is_sign_in_err.msg,
                status: is_sign_in_err.code,
            });
        }

        Err(AuthError::internal())
    }

    pub async fn get_user(headers: &HeaderMap) -> Result<UserModel, AuthError> {
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
            .header(HeaderType::AUTHORIZATION, &token)
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

        if let Ok(is_token_valid) = serde_json::from_str::<GetUserSuccessResponse>(&res) {
            let user_model_parsing = UserService::get_user_by_id(is_token_valid.id).await;
            if let Ok(data) = user_model_parsing {
                return Ok(data);
            }
            return Err(AuthError::internal());
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
