use crate::{
    models::disaster_aid_model::CreateDisasterAid,
    service::disaster_report_aid_service::DiassterReportAidService,
    utils::{
        request::json_parser::JsonParser,
        responses::api_responses::{ApiResponse, ApiResponseReturnTypeWithHeader},
    },
};

pub struct DisasterAidHandler;

impl DisasterAidHandler {
    pub async fn create(
        JsonParser(payload): JsonParser<CreateDisasterAid>,
    ) -> ApiResponseReturnTypeWithHeader<bool> {
        let service = DiassterReportAidService::create(payload).await;
        match service {
            Err(err) => err.to_handler_error(),
            Ok(_) => ApiResponse::success(None, None, None),
        }
    }
}
