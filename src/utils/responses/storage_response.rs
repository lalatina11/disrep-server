use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::utils::responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResultURL {
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupabaseStorageResult {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Id")]
    pub id: String,
}

impl SupabaseStorageResult {
    pub fn into_response(self) -> ApiResponseReturnTypeWithHeader<StorageResultURL> {
        ApiResponse::success(
            Some(StorageResultURL { url: self.key }),
            Some("Success to upload file!".to_string()),
            Some(StatusCode::CREATED),
        )
    }
}
