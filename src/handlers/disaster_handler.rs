use axum::{
    Extension,
    extract::{Multipart, Path},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    middleware::admin_middleware::ALLOWED_ADMIN_ROLES,
    models::{disaster_model::DisasterReportsModel, user_model::UserModel},
    service::disaster_service::DisasterService,
    utils::responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader},
};

pub struct DisasterHandler;

impl DisasterHandler {
    pub async fn get_all() -> impl IntoResponse {
        let service = DisasterService::get_all();
        match service {
            Ok(data) => ApiResponse::success(Some(data), None, None),
            Err(err) => err.to_handler_error(),
        }
    }

    pub async fn create(
        Extension(user): Extension<UserModel>,
        multipart: Multipart,
    ) -> ApiResponseReturnTypeWithHeader<DisasterReportsModel> {
        let service = DisasterService::create(user.id, multipart).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(disaster) => ApiResponse::success(Some(disaster), None, None),
        }
    }

    pub async fn get_by_id(
        authenticated: Option<Extension<UserModel>>,
        Path(id): Path<uuid::Uuid>,
    ) -> ApiResponseReturnTypeWithHeader<DisasterReportsModel> {
        let authenticated = authenticated.map(|Extension(user)| user);
        let service = DisasterService::get_by_id(id).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(disaster) => {
                if disaster.status != "new" {
                    return ApiResponse::success(Some(disaster), None, None);
                }
                if let Some(user) = authenticated {
                    if disaster.status == "new" && ALLOWED_ADMIN_ROLES.contains(&user.role.as_str())
                    {
                        return ApiResponse::success(Some(disaster), None, None);
                    }
                }
                let status = StatusCode::NOT_FOUND;
                ApiResponse::error(Some(status.to_string()), Some(status))
            }
        }
    }

    pub async fn approve(
        Path(id): Path<uuid::Uuid>,
    ) -> ApiResponseReturnTypeWithHeader<DisasterReportsModel> {
        let service = DisasterService::approve(id).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(disaster) => ApiResponse::success(Some(disaster), None, None),
        }
    }
}
