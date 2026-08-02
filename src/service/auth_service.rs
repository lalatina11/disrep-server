use std::str::FromStr;

use axum::http::{HeaderMap, header as HeaderType};
use reqwest::Client;
use uuid::Uuid;

use crate::{
    config::supabase_config::SupabaseConfig,
    error::ServiceError,
    models::{
        auth_model::{AuthPayload, SignInPayload, SignUpPayload},
        user_model::{NewUser, UserModel},
    },
    service::user_service::UserService,
    utils::responses::auth_responses::{
        GetUserSuccessResponse, SignUpAndInSuccessResponse, SupabaseAuthErrorResponse,
    },
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_up(payload: SignUpPayload) -> Result<AuthPayload, ServiceError> {
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

        if let Ok(is_sign_up_success) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            let user_model_parsing = UserService::create_user(NewUser {
                display_name: is_sign_up_success.user.user_metadata.display_name,
                email: is_sign_up_success.user.email,
                id: Uuid::from_str(&is_sign_up_success.user.id).unwrap_or(uuid::Uuid::new_v4()),
            })
            .await;
            if let Ok(user_model) = user_model_parsing {
                return Ok(user_model.to_payload(is_sign_up_success.access_token));
            }
            return Err(ServiceError::internal());
        }

        if let Ok(is_sign_up_err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(ServiceError {
                message: is_sign_up_err.msg,
                status: is_sign_up_err.code,
            });
        }

        Err(ServiceError::internal())
    }

    pub async fn sign_in(payload: SignInPayload) -> Result<AuthPayload, ServiceError> {
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
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                ServiceError::internal()
            })?;

        if let Ok(is_sign_in_success) = serde_json::from_str::<SignUpAndInSuccessResponse>(&res) {
            let user_model_parsing = UserService::get_user_by_id(
                Uuid::from_str(&is_sign_in_success.user.id).unwrap_or(Uuid::new_v4()),
            )
            .await;
            if let Ok(user_model) = user_model_parsing {
                return Ok(user_model.to_payload(is_sign_in_success.access_token));
            }
            return Err(ServiceError::internal());
        }

        if let Ok(is_sign_in_err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(ServiceError {
                message: is_sign_in_err.msg,
                status: is_sign_in_err.code,
            });
        }

        Err(ServiceError::internal())
    }

    pub async fn get_user(headers: &HeaderMap) -> Result<UserModel, ServiceError> {
        let token = headers
            .get(HeaderType::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or("".to_string());

        if token == "" {
            return Err(ServiceError {
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
                println!("Error while getting user: {}", err);
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|err| {
                println!("Error while parsing body: {}", err);
                ServiceError::internal()
            })?;

        if let Ok(is_token_valid) = serde_json::from_str::<GetUserSuccessResponse>(&res) {
            let user_model_parsing = UserService::get_user_by_id(
                Uuid::from_str(&is_token_valid.id).unwrap_or(Uuid::new_v4()),
            )
            .await;
            if let Ok(data) = user_model_parsing {
                return Ok(data);
            }
            return Err(ServiceError::internal());
        }

        if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res) {
            return Err(ServiceError {
                message: err.msg,
                status: err.code,
            });
        }

        Err(ServiceError::internal())
    }
}
