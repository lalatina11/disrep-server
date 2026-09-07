use std::str::FromStr;
use validator::Validate;

use uuid::Uuid;

use crate::{
    config::server_config::AppEnv,
    error::{ServiceError, supabase_error::SupabaseAuthErrorResponse},
    models::{
        auth_model::{
            AuthPayload, AuthToken, RefreshTokenPayload, SignInPayload, SignUpAdditionalData,
            SignUpPayload,
        },
        auth_update_user_model::{
            AdditionalData, AuthUpdateUserPayload, UpdatePasswordPayload, UpdateProfilePayload,
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

    pub async fn get_user(token: String) -> Result<UserModel, ServiceError> {
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

    pub fn generate_cookie(cookie_name: &str, token: String, day: Option<u8>) -> String {
        let days = day.unwrap_or(3) as u64;
        let max_age = days * 24 * 60 * 60;
        let is_production = AppEnv::new() == AppEnv::Production;
        let secure_flag = if is_production { "; Secure" } else { "" };

        format!(
            "{}={}; Path=/; HttpOnly; Max-Age={}; SameSite=Lax{}",
            cookie_name, token, max_age, secure_flag,
        )
    }

    pub fn generate_clear_cookie(cookie_name: &str) -> String {
        Self::generate_cookie(cookie_name, "".to_string(), Some(0))
    }

    pub async fn refresh_token(payload: RefreshTokenPayload) -> Result<AuthPayload, ServiceError> {
        payload.validate()?;
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

    pub async fn sign_out(access_token: String) -> Result<(), ServiceError> {
        SupabaseService::sign_out(access_token).await
    }

    pub async fn update_profile(
        token: String,
        user: UserModel,
        payload: AuthUpdateUserPayload,
    ) -> Result<UserModel, ServiceError> {
        let cloned_current_user = UserModel {
            id: user.id,
            display_name: user.display_name.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            avatar: user.avatar.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        };
        let avatar = payload.data.avatar.unwrap_or(
            user.avatar
                .unwrap_or(UserService::generate_avatar(&user.display_name)),
        );
        let display_name = payload.data.display_name.unwrap_or(user.display_name);
        let update_profile_payload = UpdateProfilePayload {
            display_name,
            avatar,
        };

        update_profile_payload.validate()?;

        let update_profile_res =
            SupabaseService::update_user(token, update_profile_payload.to_update_user_payload())
                .await?;

        let update_user_res = UserService::update_user(
            user.id,
            update_profile_res.to_new_user_payload(cloned_current_user),
        )
        .await?;

        Ok(update_user_res)
    }

    pub async fn update_password(
        token: String,
        UpdatePasswordPayload {
            current_password,
            new_password,
        }: UpdatePasswordPayload,
    ) -> Result<(), ServiceError> {
        let user = Self::get_user(token.clone()).await?;
        let try_login = Self::sign_in(SignInPayload {
            email: user.email,
            password: current_password,
        })
        .await;
        if let Err(_) = try_login {
            return Err(ServiceError::unprocessable(Some(
                "Invalid Password!".to_string(),
            )));
        }
        if let Ok(login_data) = try_login {
            let payload =
                AuthUpdateUserPayload {
                    password: Some(new_password),
                    data: AdditionalData {
                        avatar: Some(login_data.user.avatar.unwrap_or(
                            UserService::generate_avatar(&login_data.user.display_name),
                        )),
                        display_name: Some(login_data.user.display_name),
                    },
                };
            let res = SupabaseService::update_user(token, payload).await;
            return match res {
                Ok(_) => {
                    Self::sign_out(login_data.token.access_token).await?;
                    Ok(())
                }
                Err(err) => Err(err),
            };
        }
        Err(ServiceError::internal())
    }
}
