use reqwest::StatusCode;
use serde::Serialize;

use crate::utils::responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader};

pub mod supabase_error;

#[derive(Debug)]
pub struct ServiceError {
    pub message: String,
    pub status: u16,
}

impl ServiceError {
    pub fn internal() -> Self {
        Self {
            message: "An unexpected error occurred".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    pub fn unauthorized(msg: Option<String>) -> Self {
        let status = StatusCode::UNAUTHORIZED;
        Self {
            message: msg.unwrap_or_else(|| status.to_string()),
            status: status.as_u16(),
        }
    }
    pub fn unprocessable(msg: Option<String>) -> Self {
        let status = StatusCode::UNPROCESSABLE_ENTITY;
        Self {
            message: msg.unwrap_or_else(|| status.to_string()),
            status: status.as_u16(),
        }
    }

    pub fn not_found(msg: Option<String>) -> Self {
        let status = StatusCode::NOT_FOUND;
        Self {
            message: msg.unwrap_or_else(|| status.to_string()),
            status: status.as_u16(),
        }
    }

    pub fn get_status(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<axum::extract::multipart::MultipartError> for ServiceError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        Self {
            message: err.to_string(),
            status: StatusCode::BAD_REQUEST.as_u16(),
        }
    }
}

impl From<validator::ValidationErrors> for ServiceError {
    fn from(err: validator::ValidationErrors) -> Self {
        let message = if let Some((field, errors)) = err.field_errors().into_iter().next() {
            if let Some(first_err) = errors.first() {
                if let Some(ref msg) = first_err.message {
                    msg.to_string()
                } else {
                    format!("Invalid {}", field)
                }
            } else {
                "Invalid input".to_string()
            }
        } else {
            "Invalid input".to_string()
        };

        Self::unprocessable(Some(message))
    }
}

impl ServiceError {
    pub fn to_handler_error<T: Serialize>(&self) -> ApiResponseReturnTypeWithHeader<T> {
        ApiResponse::error(
            Some(self.message.clone()),
            Some(StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)),
        )
    }
}
