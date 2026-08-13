use std::str::FromStr;
use validator::Validate;

use axum::http::{HeaderMap, header as HeaderType};
use uuid::Uuid;

use crate::{
    config::server_config::AppEnv,
    error::{ServiceError, supabase_error::SupabaseAuthErrorResponse},
    models::{
        auth_model::{
            AuthPayload, AuthToken, RefreshTokenPayload, SignInPayload, SignUpAdditionalData,
            SignUpPayload,
        },
        user_model::UserModel,
    },
    service::{supabase_service::SupabaseService, user_service::UserService},
    utils::responses::auth_responses::{GetUserSuccessResponse, SignUpAndInSuccessResponse},
};

pub struct AuthService;

impl AuthService {
    pub async fn sign_up(payload: SignUpPayload) -> Result<AuthPayload, ServiceError> {
        payload.validate()?;

        let payload = SignUpPayload {
            email: payload.email,
            password: payload.password,
            data: SignUpAdditionalData {
                role: Some("user".to_string()),
                display_name: payload.data.display_name,
            },
        };

        payload.data.validate()?;

        let res = SupabaseService::sign_up_user(payload).await;

        if let Ok(res_text) = res {
            if let Ok(is_sign_up_success) =
                serde_json::from_str::<SignUpAndInSuccessResponse>(&res_text)
            {
                let create_user = is_sign_up_success.create_user().await;
                if let Ok(user_model) = create_user {
                    let access_token = is_sign_up_success.access_token;
                    let refresh_token = is_sign_up_success.refresh_token;
                    return Ok(user_model.to_payload(AuthToken {
                        access_token,
                        refresh_token,
                    }));
                }
                return Err(ServiceError::internal());
            }

            if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res_text) {
                return Err(err.to_service_error());
            }
        }

        Err(ServiceError::internal())
    }

    pub async fn sign_in(payload: SignInPayload) -> Result<AuthPayload, ServiceError> {
        payload.validate()?;

        let res = SupabaseService::sign_in_user(payload).await;

        if let Ok(res_text) = res {
            if let Ok(is_sign_in_success) =
                serde_json::from_str::<SignUpAndInSuccessResponse>(&res_text)
            {
                let existing_user = is_sign_in_success.check_existing_user().await;
                if let Ok(user_model) = existing_user {
                    let access_token = is_sign_in_success.access_token;
                    let refresh_token = is_sign_in_success.refresh_token;
                    return Ok(user_model.to_payload(AuthToken {
                        access_token,
                        refresh_token,
                    }));
                } else {
                    let create_user = is_sign_in_success.create_user().await;
                    if let Ok(user_model) = create_user {
                        let access_token = is_sign_in_success.access_token;
                        let refresh_token = is_sign_in_success.refresh_token;
                        return Ok(user_model.to_payload(AuthToken {
                            access_token,
                            refresh_token,
                        }));
                    }
                }
                return Err(ServiceError::internal());
            }

            if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res_text) {
                return Err(err.to_service_error());
            }
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

        let res = SupabaseService::get_user(token).await;

        if let Ok(res_text) = res {
            if let Ok(is_token_valid) = serde_json::from_str::<GetUserSuccessResponse>(&res_text) {
                let user_model_parsing = UserService::get_user_by_id(
                    Uuid::from_str(&is_token_valid.id).unwrap_or(Uuid::new_v4()),
                )
                .await;
                if let Ok(data) = user_model_parsing {
                    return Ok(data);
                }
                return Err(ServiceError::internal());
            }

            if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res_text) {
                return Err(err.to_service_error());
            }
        }

        Err(ServiceError::internal())
    }

    pub fn generate_cookie(token: String) -> String {
        let max_age = 6 * 24 * 60 * 60; // 6 days in seconds (259200)
        let is_production = AppEnv::new() == AppEnv::Production;
        let secure_flag = if is_production { "; Secure" } else { "" };

        format!(
            "access_token={}; Path=/; HttpOnly; Max-Age={}; SameSite=Lax{}",
            token, max_age, secure_flag
        )
    }

    pub async fn refresh_token(payload: RefreshTokenPayload) -> Result<AuthPayload, ServiceError> {
        let res = SupabaseService::refresh_token(payload).await;

        if let Ok(res_text) = res {
            if let Ok(is_sign_in_success) =
                serde_json::from_str::<SignUpAndInSuccessResponse>(&res_text)
            {
                let existing_user = is_sign_in_success.check_existing_user().await;
                if let Ok(user_model) = existing_user {
                    let access_token = is_sign_in_success.access_token;
                    let refresh_token = is_sign_in_success.refresh_token;
                    return Ok(user_model.to_payload(AuthToken {
                        access_token,
                        refresh_token,
                    }));
                } else {
                    let create_user = is_sign_in_success.create_user().await;
                    if let Ok(user_model) = create_user {
                        let access_token = is_sign_in_success.access_token;
                        let refresh_token = is_sign_in_success.refresh_token;
                        return Ok(user_model.to_payload(AuthToken {
                            access_token,
                            refresh_token,
                        }));
                    }
                }
                return Err(ServiceError::internal());
            }

            if let Ok(err) = serde_json::from_str::<SupabaseAuthErrorResponse>(&res_text) {
                return Err(err.to_service_error());
            }
        }

        Err(ServiceError::internal())
    }
}
