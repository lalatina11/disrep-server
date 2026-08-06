use axum::{Extension, extract::Multipart, response::IntoResponse};

use crate::{
    models::{disaster_model::CreateDisasterReportPayload, user_model::UserModel},
    service::disaster_service::DisasterService,
    utils::{
        request::json_parser::JsonParser,
        responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader},
    },
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
        JsonParser(payload): JsonParser<CreateDisasterReportPayload>,
    ) -> impl IntoResponse {
        let service = DisasterService::create(payload.into_record(user.id));
        match service {
            Ok(data) => ApiResponse::success(Some(data), None, None),
            Err(err) => err.to_handler_error(),
        }
    }

    pub async fn supabase_upload(
        Extension(user): Extension<UserModel>,

        multipart: Multipart,
    ) -> ApiResponseReturnTypeWithHeader<bool> {
        let service = DisasterService::upload(user.id, multipart).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(_) => ApiResponse::success(None, None, None),
        }
    }
}
