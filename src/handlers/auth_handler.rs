use axum::{
    Extension, Json,
    http::{HeaderMap, HeaderValue, header as HeaderType},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    models::{
        auth_model::{AuthPayload, RefreshTokenPayload, SignInPayload, SignUpPayload},
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
            let access_token = AuthService::generate_cookie(data.access_token.clone());
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
            return err.to_handler_error();
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
            let access_token = AuthService::generate_cookie(data.access_token.clone());
            headers.insert(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            return (
                StatusCode::OK,
                headers,
                Json::<ApiResponse<AuthPayload>>(ApiResponse {
                    success: true,
                    message: "Login user success".to_string(),
                    data: Some(data),
                }),
            );
        } else if let Err(err) = service {
            return err.to_handler_error();
        }
        ApiResponse::error(None, None)
    }

    pub async fn get_user(
        Extension(data): Extension<UserModel>,
    ) -> ApiResponseReturnTypeWithHeader<UserModel> {
        ApiResponse::success(Some(data), None, Some(StatusCode::OK))
    }

    pub async fn refresh_token(
        JsonParser(payload): JsonParser<RefreshTokenPayload>,
    ) -> ApiResponseReturnTypeWithHeader<AuthPayload> {
        let service = AuthService::refresh_token(payload).await;
        let mut headers = HeaderMap::new();
        headers.insert(HANDLED_HEADER, HeaderValue::from_static("true"));
        if let Ok(data) = service {
            let access_token = AuthService::generate_cookie(data.access_token.clone());
            headers.insert(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            return (
                StatusCode::OK,
                headers,
                Json::<ApiResponse<AuthPayload>>(ApiResponse {
                    success: true,
                    message: "Login user success".to_string(),
                    data: Some(data),
                }),
            );
        } else if let Err(err) = service {
            return err.to_handler_error();
        }
        ApiResponse::error(None, None)
    }
}
