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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSupabaseResult {
    pub media_url: String,
    pub media_id: String,
}

impl SupabaseStorageResult {
    pub fn parsed(self) -> ParsedSupabaseResult {
        ParsedSupabaseResult {
            media_url: self.key,
            media_id: self.id,
        }
    }
}

impl ParsedSupabaseResult {
    pub fn into_response(self) -> ApiResponseReturnTypeWithHeader<ParsedSupabaseResult> {
        ApiResponse::success(
            Some(self),
            Some("Success to upload file!".to_string()),
            Some(StatusCode::CREATED),
        )
    }
}
