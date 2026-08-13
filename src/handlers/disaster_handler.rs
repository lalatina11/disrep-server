use axum::{Extension, extract::Path};
use reqwest::StatusCode;

use crate::{
    models::{
        disaster_model::{CreateDisasterReportWithImage, DisasterReportsModel},
        user_model::UserModel,
    },
    service::disaster_service::{DisasterService, DisasterWithAllRelations},
    utils::{
        request::json_parser::JsonParser,
        responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader},
    },
};

pub struct DisasterHandler;

impl DisasterHandler {
    pub async fn get_all() -> ApiResponseReturnTypeWithHeader<Vec<DisasterWithAllRelations>> {
        let service = DisasterService::get_all();
        match service {
            Ok(data) => ApiResponse::success(Some(data), None, None),
            Err(err) => err.to_handler_error(),
        }
    }

    pub async fn create(
        Extension(user): Extension<UserModel>,
        JsonParser(payload): JsonParser<CreateDisasterReportWithImage>,
    ) -> ApiResponseReturnTypeWithHeader<DisasterReportsModel> {
        let service = DisasterService::create(user.id, payload).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(disaster) => ApiResponse::success(
                Some(disaster),
                Some("Success to crete disaster".to_string()),
                Some(StatusCode::CREATED),
            ),
        }
    }

    pub async fn get_by_id(
        authenticated: Option<Extension<UserModel>>,
        Path(id): Path<uuid::Uuid>,
    ) -> ApiResponseReturnTypeWithHeader<DisasterWithAllRelations> {
        let authenticated = authenticated.map(|Extension(user)| user);
        let service = DisasterService::get_by_id(id).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(data) => {
                if data.disaster.status != "pending" {
                    return ApiResponse::success(Some(data), None, None);
                }
                if let Some(user) = authenticated {
                    if data.disaster.status == "pending" && user.is_authorize_as_admin() {
                        return ApiResponse::success(Some(data), None, None);
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
