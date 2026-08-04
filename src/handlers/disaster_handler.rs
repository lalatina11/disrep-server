use axum::response::IntoResponse;

use crate::utils::responses::api_responses::ApiResponse;

pub struct DisasterHandler;

impl DisasterHandler {
    pub async fn get_all() -> impl IntoResponse {
        ApiResponse::success(Some(1), None, None)
    }
}
