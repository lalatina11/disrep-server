use axum::{extract::Request, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    service::auth_service::AuthService,
    utils::{CommonUtility, responses::api_responses::ApiResponse},
};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn handle(mut req: Request, next: Next) -> impl IntoResponse {
        let token = CommonUtility::get_access_token_from_headers(req.headers());
        let user_payload = AuthService::get_user(token).await;

        match user_payload {
            Err(_) => {
                let status = StatusCode::UNAUTHORIZED;
                ApiResponse::<bool>::error(Some(status.to_string()), Some(status)).into_response()
            }
            Ok(data) => {
                req.extensions_mut().insert(data.fix_avatar_url());
                next.run(req).await.into_response()
            }
        }
    }

    pub async fn optional(mut req: Request, next: Next) -> impl IntoResponse {
        let token = CommonUtility::get_access_token_from_headers(req.headers());
        let user_payload = AuthService::get_user(token).await;
        if let Ok(data) = user_payload {
            req.extensions_mut().insert(data.fix_avatar_url());
        }
        next.run(req).await.into_response()
    }
}
