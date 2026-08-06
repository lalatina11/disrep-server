use axum::extract::Multipart;
use reqwest::{Client, header as HeaderType};

use crate::{
    config::supabase_config::SupabaseConfig,
    error::ServiceError,
    models::auth_model::{SignInPayload, SignUpPayload},
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

    pub async fn upload_image(mut multipart: Multipart) -> Result<(), ServiceError> {
        while let Some(field) = multipart.next_field().await? {
            let name = field.name().unwrap_or("");

            let bytes = match name {
                "image" => {
                    let filename = field.file_name().unwrap_or("image.jpg").to_string();

                    let _bytes = field.bytes().await?;

                    println!("{filename}");
                    println!("{} bytes", _bytes.len());
                    Ok(_bytes)
                }

                _ => Err(ServiceError::internal()),
            };
        }

        Err(ServiceError::internal())
    }
}
