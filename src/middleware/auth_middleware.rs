use axum::{extract::Request, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    models::auth_model::UserPayload, service::auth_service::AuthService,
    utils::responses::api_responses::ApiResponse,
};

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
                req.extensions_mut().insert(UserPayload {
                    app_metadata: data.app_metadata,
                    aud: data.aud,
                    confirmed_at: data.confirmed_at,
                    created_at: data.created_at,
                    email: data.email,
                    email_confirmed_at: data.email_confirmed_at,
                    id: data.id,
                    identities: data.identities,
                    is_anonymous: data.is_anonymous,
                    last_sign_in_at: data.last_sign_in_at,
                    phone: data.phone,
                    role: data.role,
                    updated_at: data.updated_at,
                    user_metadata: data.user_metadata,
                });
                next.run(req).await.into_response()
            }
        }
    }
}
