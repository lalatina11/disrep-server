use axum::Extension;
use reqwest::StatusCode;

use crate::{
    models::{
        auth_model::{AuthPayload, SignInPayload, SignUpPayload},
        user_model::UserModel,
    },
    service::auth_service::AuthService,
    utils::{
        request::json_parser::JsonParser,
        responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader},
    },
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn sign_up(
        JsonParser(payload): JsonParser<SignUpPayload>,
    ) -> ApiResponseReturnTypeWithHeader<AuthPayload> {
        let service = AuthService::sign_up(payload).await;
        match service {
            Err(err) => ApiResponse::error(
                Some(err.message.to_string()),
                Some(StatusCode::from_u16(err.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)),
            ),
            Ok(data) => ApiResponse::success(Some(data), None, Some(StatusCode::CREATED)),
        }
    }

    pub async fn sign_in(
        JsonParser(payload): JsonParser<SignInPayload>,
    ) -> ApiResponseReturnTypeWithHeader<AuthPayload> {
        let service = AuthService::sign_in(payload).await;
        match service {
            Err(err) => ApiResponse::error(
                Some(err.message),
                Some(StatusCode::from_u16(err.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)),
            ),
            Ok(data) => ApiResponse::success(Some(data), None, None),
        }
    }

    pub async fn get_user(
        Extension(data): Extension<UserModel>,
    ) -> ApiResponseReturnTypeWithHeader<UserModel> {
        ApiResponse::success(Some(data), None, Some(StatusCode::OK))
    }
}
