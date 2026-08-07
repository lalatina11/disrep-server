use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseAuthErrorResponse {
    pub code: u16,
    pub error_code: String,
    pub msg: String,
}

impl SupabaseAuthErrorResponse {
    pub fn to_service_error(self) -> ServiceError {
        ServiceError {
            message: self.msg,
            status: self.code,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseStorageErrorResponse {
    #[serde(rename = "statusCode")]
    pub status_code: String,
    pub error: String,
    pub message: String,
    pub code: String,
}

impl SupabaseStorageErrorResponse {
    pub fn to_service_error(&self) -> ServiceError {
        ServiceError {
            message: self.message.clone(),
            status: self.status_code.parse().unwrap_or(400),
        }
    }
}
