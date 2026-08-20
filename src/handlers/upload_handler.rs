use axum::extract::Multipart;

use crate::{
    service::upload_service::UploadService,
    utils::responses::{
        api_responses::ApiResponseReturnTypeWithHeader, storage_response::ParsedSupabaseResult,
    },
};

pub struct UploadHandler;

impl UploadHandler {
    pub async fn image(
        multipart: Multipart,
    ) -> ApiResponseReturnTypeWithHeader<ParsedSupabaseResult> {
        let service = UploadService::upload_image(multipart).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(res) => res.parsed().into_response(),
        }
    }
    pub async fn video(
        multipart: Multipart,
    ) -> ApiResponseReturnTypeWithHeader<ParsedSupabaseResult> {
        let service = UploadService::upload_video(multipart).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(res) => res.parsed().into_response(),
        }
    }
}
