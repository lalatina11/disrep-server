use axum::extract::Multipart;

use crate::{
    error::ServiceError, models::form_data::FileFormData,
    service::supabase_service::SupabaseService,
    utils::responses::storage_response::SupabaseStorageResult,
};

pub struct UploadService;

impl UploadService {
    pub async fn parse_image_multipart(
        mut multipart: Multipart,
    ) -> Result<FileFormData, ServiceError> {
        let mut file = FileFormData {
            bytes: None,
            content_type: None,
            name: "".to_string(),
        };

        while let Some(field) = multipart.next_field().await? {
            let form_key = field.name().unwrap_or("");

            match form_key {
                "image" => {
                    file.name = field.file_name().unwrap_or("no_name").to_string();
                    let content_type = field.content_type().map(|s| s.to_string());
                    file.content_type = content_type;
                    let bytes = field.bytes().await?;
                    file.bytes = Some(bytes)
                }
                _ => {}
            }
        }

        Ok(file)
    }

    pub async fn parse_video_multipart(
        mut multipart: Multipart,
    ) -> Result<FileFormData, ServiceError> {
        let mut file = FileFormData {
            bytes: None,
            content_type: None,
            name: "".to_string(),
        };

        while let Some(field) = multipart.next_field().await? {
            let form_key = field.name().unwrap_or("");

            match form_key {
                "video" => {
                    file.name = field.file_name().unwrap_or("no_name").to_string();
                    let content_type = field.content_type().map(|s| s.to_string());
                    file.content_type = content_type;
                    let bytes = field.bytes().await?;
                    file.bytes = Some(bytes)
                }
                _ => {}
            }
        }

        Ok(file)
    }

    pub async fn upload_image(multipart: Multipart) -> Result<SupabaseStorageResult, ServiceError> {
        let buff = Self::parse_image_multipart(multipart).await;

        if let Err(err) = &buff {
            println!("Error while uploading image:\n{:?}", err);
        }
        if let Ok(file) = buff {
            return SupabaseService::upload_file(file).await;
        }
        Err(ServiceError::internal())
    }
}
