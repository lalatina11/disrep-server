use axum::{
    Extension, Json,
    http::{HeaderMap, HeaderValue, header as HeaderType},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    models::{
        auth_model::{AuthPayload, SignInPayload, SignUpPayload},
        user_model::UserModel,
    },
    service::auth_service::AuthService,
    utils::{
        request::json_parser::JsonParser,
        responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader, HANDLED_HEADER},
    },
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn sign_up(JsonParser(payload): JsonParser<SignUpPayload>) -> impl IntoResponse {
        let service = AuthService::sign_up(payload).await;
        let mut headers = HeaderMap::new();
        headers.insert(HANDLED_HEADER, HeaderValue::from_static("true"));
        if let Ok(data) = service {
            let access_token = format!("access_token={};httpOnly=true;", data.access_token);
            headers.insert(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            return (
                StatusCode::CREATED,
                headers,
                Json::<ApiResponse<AuthPayload>>(ApiResponse {
                    success: true,
                    message: "Register user success".to_string(),
                    data: Some(data),
                }),
            );
        } else if let Err(err) = service {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                headers,
                Json::<ApiResponse<AuthPayload>>(ApiResponse {
                    success: false,
                    message: err.message.to_string(),
                    data: None,
                }),
            );
        }
        ApiResponse::error(None, None)
    }

    pub async fn sign_in(
        JsonParser(payload): JsonParser<SignInPayload>,
    ) -> ApiResponseReturnTypeWithHeader<AuthPayload> {
        let service = AuthService::sign_in(payload).await;
        let mut headers = HeaderMap::new();
        headers.insert(HANDLED_HEADER, HeaderValue::from_static("true"));
        if let Ok(data) = service {
            let access_token = format!("access_token={};httpOnly=true;", data.access_token);
            headers.insert(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            return (
                StatusCode::CREATED,
                headers,
                Json::<ApiResponse<AuthPayload>>(ApiResponse {
                    success: true,
                    message: "Login user success".to_string(),
                    data: Some(data),
                }),
            );
        } else if let Err(err) = service {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                headers,
                Json::<ApiResponse<AuthPayload>>(ApiResponse {
                    success: false,
                    message: err.message.to_string(),
                    data: None,
                }),
            );
        }
        ApiResponse::error(None, None)
    }

    pub async fn get_user(
        Extension(data): Extension<UserModel>,
    ) -> ApiResponseReturnTypeWithHeader<UserModel> {
        ApiResponse::success(Some(data), None, Some(StatusCode::OK))
    }
}
