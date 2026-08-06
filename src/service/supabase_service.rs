use axum::extract::Multipart;
use reqwest::{Client, header as HeaderType};

use crate::{
    config::supabase_config::SupabaseConfig,
    error::{ServiceError, supabase_error::SupabaseStorageErrorResponse},
    models::{
        auth_model::{SignInPayload, SignUpPayload},
        disaster_model::CreateDisasterReportPayload,
        form_data::ImageFormData,
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

    pub async fn upload_image(
        mut multipart: Multipart,
    ) -> Result<CreateDisasterReportPayload, ServiceError> {
        let mut payload = CreateDisasterReportPayload {
            title: "".to_string(),
            description: Some("".to_string()),
            street: Some("".to_string()),
            city: "".to_string(),
            lat: 0.0,
            lng: 0.0,
            image: "".to_string(),
            image_storage_url: "".to_string(),
        };
        let mut image = ImageFormData {
            name: "".to_string(),
            bytes: None,
            content_type: None,
        };
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("");

            match name {
                "image" => {
                    let filename = field.file_name().unwrap_or("image.jpg").to_string();
                    let content_type = field.content_type().map(|s| s.to_string());
                    image.name = filename;
                    image.content_type = content_type;
                    let bytes = field.bytes().await?;
                    image.bytes = Some(bytes);
                }
                "title" => {
                    payload.title = field.text().await.unwrap_or("no title".to_string());
                }
                "description" => {
                    let description = field.text().await;
                    payload.description = match description {
                        Ok(desc) => Some(desc),
                        Err(_) => None,
                    }
                }
                "street" => {
                    let street = field.text().await;
                    payload.street = match street {
                        Ok(desc) => Some(desc),
                        Err(_) => None,
                    }
                }
                "city" => {
                    payload.city = field.text().await.unwrap_or("no city".to_string());
                }
                "lat" => {
                    let string_lat = field.text().await.unwrap_or("".to_string());
                    let _lat: f64 = string_lat.parse().unwrap_or(0.0);
                    payload.lat = _lat;
                }
                "lng" => {
                    let string_lng = field.text().await.unwrap_or("".to_string());
                    let _lng: f64 = string_lng.parse().unwrap_or(0.0);
                    payload.lng = _lng;
                }

                _ => {}
            };
        }
        match image.bytes {
            None => Err(ServiceError::internal()),
            Some(bytes) => {
                let supabase = SupabaseConfig::new();
                let image_name = CommonUtility::generate_unique_name();
                let client = Client::new();
                let url = format!(
                    "{}/object/disaster-image/{}",
                    supabase.storage_base_url, image_name
                );

                // let content_type = match image.content_type.as_deref() {
                //     Some("application/octet-stream") | None => {
                //         let lower_name = image.name.to_lowercase();
                //         if lower_name.ends_with(".png") {
                //             "image/png"
                //         } else if lower_name.ends_with(".webp") {
                //             "image/webp"
                //         } else if lower_name.ends_with(".gif") {
                //             "image/gif"
                //         } else {
                //             "image/jpeg"
                //         }
                //     }
                //     Some(ct) => ct,
                // };

                let res_text = client
                    .post(url)
                    .header("apikey", supabase.publishable_key)
                    .header(HeaderType::AUTHORIZATION, supabase.admin_token)
                    .header(HeaderType::CACHE_CONTROL, 3600)
                    .header(HeaderType::CONTENT_TYPE, "image/*")
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
                    Ok(res) => {
                        payload.image = CommonUtility::generate_image_url(res.key.clone());
                        payload.image_storage_url = res.key;
                        Ok(payload)
                    }
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
