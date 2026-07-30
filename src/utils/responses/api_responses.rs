use axum::{
    Json,
    http::{HeaderMap, HeaderValue, StatusCode},
};
use serde::Serialize;

pub type ApiResponseReturnType<T> = (StatusCode, Json<ApiResponse<T>>);
pub type ApiResponseReturnTypeWithHeader<T> = (StatusCode, HeaderMap, Json<ApiResponse<T>>);

pub const HANDLED_HEADER: &str = "x-handled-header";

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
    ) -> ApiResponseReturnTypeWithHeader<T> {
        let status = status.unwrap_or_else(|| StatusCode::OK);
        let mut headers = HeaderMap::new();
        headers.insert(HANDLED_HEADER, HeaderValue::from_static("true"));
        (
            status,
            headers,
            Json(ApiResponse {
                success: true,
                message: message.unwrap_or_else(|| status.to_string()),
                data: data,
            }),
        )
    }
    pub fn error(
        message: Option<String>,
        status: Option<StatusCode>,
    ) -> ApiResponseReturnTypeWithHeader<T> {
        let status = status.unwrap_or_else(|| StatusCode::INTERNAL_SERVER_ERROR);
        let message = message.unwrap_or(status.to_string());
        let mut headers = HeaderMap::new();
        headers.insert(HANDLED_HEADER, HeaderValue::from_static("true"));
        (
            status,
            headers,
            Json(ApiResponse {
                success: false,
                message,
                data: None,
            }),
        )
    }
}
