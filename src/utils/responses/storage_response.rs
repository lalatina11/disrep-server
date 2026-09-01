use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::utils::{
    CommonUtility,
    responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader},
};

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
    pub media_type: String,
    pub media_preview: String,
}

impl SupabaseStorageResult {
    pub fn parsed(self, media_type: String) -> ParsedSupabaseResult {
        let media_preview = CommonUtility::generate_media_url(self.key.clone());
        ParsedSupabaseResult {
            media_url: self.key,
            media_id: self.id,
            media_type,
            media_preview,
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
