use axum::Json;
use reqwest::StatusCode;

use crate::{
    models::auth_model::SignUpPayload,
    service::auth_service::AuthService,
    utils::responses::{api_responses::ApiResponse, auth_responses::SignUpAndInSuccessResponse},
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn sign_up(
        Json(payload): Json<SignUpPayload>,
    ) -> (StatusCode, Json<ApiResponse<SignUpAndInSuccessResponse>>) {
        let res = AuthService::sign_up(payload).await;
        match res {
            Err(err) => ApiResponse::error(Some(err.message), Some(err.status)),
            Ok(data) => ApiResponse::success(Some(data), None, Some(StatusCode::CREATED)),
        }
    }
}
