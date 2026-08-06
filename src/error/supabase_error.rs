use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseAuthErrorResponse {
    pub code: u16,
    pub error_code: String,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseStorageErrorResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub error: String,
    pub message: String,
    pub code: String,
}
