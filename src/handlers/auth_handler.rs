use axum::{
    Extension, Json,
    http::{HeaderMap, HeaderValue, header as HeaderType},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    models::{
        auth_model::{AuthPayload, RefreshTokenPayload, SignInPayload, SignUpPayload},
        auth_update_user_model::{AuthUpdateUserPayload, UpdatePasswordPayload},
        user_model::UserModel,
    },
    service::auth_service::AuthService,
    utils::{
        CommonUtility,
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
            let access_token =
                AuthService::generate_cookie("access_token", data.token.access_token.clone(), None);
            let refresh_token = AuthService::generate_cookie(
                "refresh_token",
                data.token.refresh_token.clone(),
                Some(7),
            );
            headers.append(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            headers.append(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&refresh_token).unwrap(),
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
            let access_token =
                AuthService::generate_cookie("access_token", data.token.access_token.clone(), None);
            let refresh_token = AuthService::generate_cookie(
                "refresh_token",
                data.token.refresh_token.clone(),
                Some(7),
            );
            headers.append(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            headers.append(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&refresh_token).unwrap(),
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
            let access_token =
                AuthService::generate_cookie("access_token", data.token.access_token.clone(), None);
            let refresh_token = AuthService::generate_cookie(
                "refresh_token",
                data.token.refresh_token.clone(),
                Some(7),
            );
            headers.append(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&access_token).unwrap(),
            );
            headers.append(
                HeaderType::SET_COOKIE,
                HeaderValue::from_str(&refresh_token).unwrap(),
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

    pub async fn sign_out(header_map: HeaderMap) -> impl IntoResponse {
        if let Some(access_token) = header_map
            .get(HeaderType::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
        {
            if let Err(err) = AuthService::sign_out(access_token.to_string()).await {
                return err.to_handler_error();
            }
        }

        let mut headers = HeaderMap::new();
        headers.insert(HANDLED_HEADER, HeaderValue::from_static("true"));
        let access_token_cookie = AuthService::generate_clear_cookie("access_token");
        let refresh_token_cookie = AuthService::generate_clear_cookie("refresh_token");
        headers.append(
            HeaderType::SET_COOKIE,
            HeaderValue::from_str(&access_token_cookie).unwrap(),
        );
        headers.append(
            HeaderType::SET_COOKIE,
            HeaderValue::from_str(&refresh_token_cookie).unwrap(),
        );
        (
            StatusCode::OK,
            headers,
            Json::<ApiResponse<AuthPayload>>(ApiResponse {
                success: true,
                message: "Sign out success".to_string(),
                data: None,
            }),
        )
    }

    pub async fn update_profile(
        headers: HeaderMap,
        Extension(user): Extension<UserModel>,
        JsonParser(payload): JsonParser<AuthUpdateUserPayload>,
    ) -> ApiResponseReturnTypeWithHeader<UserModel> {
        let token = headers
            .get(HeaderType::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or("".to_string());

        let service = AuthService::update_profile(token, user, payload).await;
        match service {
            Ok(data) => ApiResponse::success(
                Some(data),
                Some("Success to edit profile".to_string()),
                None,
            ),
            Err(err) => err.to_handler_error(),
        }
    }

    pub async fn update_password(
        headers: HeaderMap,
        JsonParser(payload): JsonParser<UpdatePasswordPayload>,
    ) -> ApiResponseReturnTypeWithHeader<()> {
        let token = CommonUtility::get_access_token_from_headers(&headers);
        let service = AuthService::update_password(token, payload).await;
        match service {
            Ok(_) => ApiResponse::success(None, Some("Update Password Success!".to_string()), None),
            Err(err) => err.to_handler_error(),
        }
    }
}
