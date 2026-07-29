use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

use crate::utils::responses::api_responses::ApiResponse;

pub struct ErrorMiddleware;

impl ErrorMiddleware {
    pub async fn generic_error(req: Request, next: Next) -> impl IntoResponse {
        let response = next.run(req).await;
        let status = response.status();
        match status {
            StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => response.into_response(),
            _other => ApiResponse::<bool>::error(None, Some(status)).into_response(),
        }
    }
}
