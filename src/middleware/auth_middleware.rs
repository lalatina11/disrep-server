use axum::{extract::Request, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;

use crate::{service::auth_service::AuthService, utils::responses::api_responses::ApiResponse};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn handle(mut req: Request, next: Next) -> impl IntoResponse {
        let user_payload = AuthService::get_user(req.headers()).await;

        match user_payload {
            Err(_) => {
                let status = StatusCode::UNAUTHORIZED;
                ApiResponse::<bool>::error(Some(status.to_string()), Some(status)).into_response()
            }
            Ok(data) => {
                req.extensions_mut().insert(data);
                next.run(req).await.into_response()
            }
        }
    }
}
