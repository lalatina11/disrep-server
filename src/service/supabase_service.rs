use reqwest::{Client, header as HeaderType};

use crate::{
    config::supabase_config::SupabaseConfig,
    error::{ServiceError, supabase_error::SupabaseStorageErrorResponse},
    models::{
        auth_model::{RefreshTokenPayload, SignInPayload, SignUpPayload},
        form_data::FileFormData,
    },
    utils::{CommonUtility, responses::storage_response::SupabaseStorageResult},
};

pub struct SupabaseService;

impl SupabaseService {
    pub async fn sign_up_user(payload: SignUpPayload) -> Result<String, ServiceError> {
        let supabase_config = SupabaseConfig::new();
        let fetch = Client::new();
        let url = format!("{}/auth/v1/signup", supabase_config.project_url);
        let res = fetch
            .post(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| {
                println!("Response error");
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                ServiceError::internal()
            })?;
        Ok(res)
    }

    pub async fn sign_in_user(payload: SignInPayload) -> Result<String, ServiceError> {
        let supabase_config = SupabaseConfig::new();
        let fetch = Client::new();
        let url = format!(
            "{}/auth/v1/token?grant_type=password",
            supabase_config.project_url
        );
        let res = fetch
            .post(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| {
                println!("Response error");
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                ServiceError::internal()
            })?;
        Ok(res)
    }
    pub async fn refresh_token(payload: RefreshTokenPayload) -> Result<String, ServiceError> {
        let supabase_config = SupabaseConfig::new();
        let fetch = Client::new();
        let url = format!(
            "{}/auth/v1/token?grant_type=refresh_token",
            supabase_config.project_url
        );
        let res = fetch
            .post(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .json(&payload)
            .send()
            .await
            .map_err(|_| {
                println!("Response error");
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|_| {
                println!("Parsing text error");
                ServiceError::internal()
            })?;
        Ok(res)
    }

    pub async fn get_user(token: String) -> Result<String, ServiceError> {
        let supabase_config = SupabaseConfig::new();
        let url = format!("{}/auth/v1/user", supabase_config.project_url);
        let fetch = Client::new();
        let res = fetch
            .get(url)
            .header(HeaderType::CONTENT_TYPE, "application/json")
            .header("apikey", supabase_config.publishable_key)
            .header(HeaderType::AUTHORIZATION, &token)
            .send()
            .await
            .map_err(|err| {
                println!("Error while getting user: {}", err);
                ServiceError::internal()
            })?
            .text()
            .await
            .map_err(|err| {
                println!("Error while parsing body: {}", err);
                ServiceError::internal()
            })?;
        Ok(res)
    }

    pub async fn upload_file(file: FileFormData) -> Result<SupabaseStorageResult, ServiceError> {
        match file.bytes {
            None => Err(ServiceError::internal()),
            Some(bytes) => {
                let supabase = SupabaseConfig::new();
                let image_name = CommonUtility::generate_unique_name();
                let client = Client::new();
                let url = format!(
                    "{}/object/disaster-report-img-vid/{}",
                    supabase.storage_base_url, image_name
                );

                let content_type = match file.content_type.as_deref() {
                    Some("application/octet-stream") | None => {
                        let lower_name = file.name.to_lowercase();
                        if lower_name.starts_with("image") {
                            if lower_name.ends_with(".png") {
                                "image/png"
                            } else if lower_name.ends_with(".webp") {
                                "image/webp"
                            } else if lower_name.ends_with(".gif") {
                                "image/gif"
                            } else {
                                "image/jpeg"
                            }
                        } else if lower_name.ends_with(".mp4") {
                            "video/mp4"
                        } else if lower_name.ends_with(".webm") {
                            "video/webm"
                        } else if lower_name.ends_with(".mov") {
                            "video/quicktime"
                        } else if lower_name.ends_with(".mkv") {
                            "video/x-matroska"
                        } else if lower_name.ends_with(".avi") {
                            "video/x-msvideo"
                        } else {
                            "video/mp4"
                        }
                    }
                    Some(ct) => ct,
                };

                let res_text = client
                    .post(url)
                    .header("apikey", supabase.publishable_key)
                    .header(HeaderType::AUTHORIZATION, supabase.admin_token)
                    .header(HeaderType::CACHE_CONTROL, 3600)
                    .header(HeaderType::CONTENT_TYPE, content_type)
                    .body(bytes)
                    .send()
                    .await
                    .map_err(|err| {
                        println!("Error while uploading image: {}", err);
                        ServiceError::internal()
                    })?
                    .text()
                    .await
                    .map_err(|err| {
                        println!("Error while parsing body: {}", err);
                        ServiceError::internal()
                    })?;

                match serde_json::from_str::<SupabaseStorageResult>(&res_text) {
                    Ok(res) => Ok(res),
                    Err(_) => match serde_json::from_str::<SupabaseStorageErrorResponse>(&res_text)
                    {
                        Ok(err) => Err(err.to_service_error()),
                        Err(_) => Err(ServiceError::internal()),
                    },
                }
            }
        }
    }
}
