use axum::{Json, http::StatusCode};
use serde::Serialize;

pub type ApiResponseReturnType<T> = (StatusCode, Json<ApiResponse<T>>);

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(
        data: Option<T>,
        message: Option<String>,
        status: Option<StatusCode>,
    ) -> ApiResponseReturnType<T> {
        let status = status.unwrap_or_else(|| StatusCode::OK);
        (
            status,
            Json(ApiResponse {
                success: true,
                message: message.unwrap_or_else(|| status.to_string()),
                data: data,
            }),
        )
    }
    pub fn error(message: Option<String>, status: Option<StatusCode>) -> ApiResponseReturnType<T> {
        let status = status.unwrap_or_else(|| StatusCode::INTERNAL_SERVER_ERROR);
        let message = message.unwrap_or(status.to_string());
        (
            status,
            Json(ApiResponse {
                success: false,
                message,
                data: None,
            }),
        )
    }
}
