use axum::response::IntoResponse;

use crate::models::api_response::ApiResponse;

pub struct RootHandler;

impl RootHandler {
    pub async fn index() -> impl IntoResponse {
        ApiResponse::<bool>::success(None, None, None)
    }
}
