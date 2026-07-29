use axum::response::IntoResponse;

use crate::utils::responses::api_responses::ApiResponse;

pub struct RootHandler;

impl RootHandler {
    pub async fn index() -> impl IntoResponse {
        ApiResponse::<bool>::success(None, None, None)
    }
}
