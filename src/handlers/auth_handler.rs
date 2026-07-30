use axum::{Json, http::HeaderMap};
use reqwest::StatusCode;

use crate::{
    models::auth_model::SignUpPayload,
    service::auth_service::AuthService,
    utils::responses::{
        api_responses::{ApiResponse, ApiResponseReturnType},
        auth_responses::SignUpAndInSuccessResponse,
    },
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn sign_up(
        Json(payload): Json<SignUpPayload>,
    ) -> ApiResponseReturnType<SignUpAndInSuccessResponse> {
        let service = AuthService::sign_up(payload).await;
        match service {
            Err(err) => ApiResponse::error(Some(err.message), Some(err.status)),
            Ok(data) => ApiResponse::success(Some(data), None, Some(StatusCode::CREATED)),
        }
    }

    pub async fn get_user(headers: HeaderMap) -> ApiResponseReturnType<String> {
        let service = AuthService::get_user(&headers).await;
        match service {
            Err(err) => ApiResponse::error(Some(err.message), Some(err.status)),
            Ok(token) => ApiResponse::success(Some(token), None, Some(StatusCode::OK)),
        }
    }
}
