use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn error(message: Option<String>, status: Option<StatusCode>) -> impl IntoResponse {
        (
            status.unwrap_or_else(|| StatusCode::INTERNAL_SERVER_ERROR),
            Json::<ApiResponse<bool>>(ApiResponse {
                success: false,
                message: message.unwrap_or_else(|| StatusCode::INTERNAL_SERVER_ERROR.to_string()),
                data: None,
            }),
        )
    }

    pub fn success(
        data: Option<T>,
        message: Option<String>,
        status: Option<StatusCode>,
    ) -> impl IntoResponse {
        (
            status.unwrap_or_else(|| StatusCode::OK),
            Json::<ApiResponse<T>>(ApiResponse {
                success: true,
                message: message.unwrap_or_else(|| StatusCode::OK.to_string()),
                data: data,
            }),
        )
    }
}
