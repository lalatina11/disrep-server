use axum::{Extension, extract::Request, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;

use crate::{models::user_model::UserModel, utils::responses::api_responses::ApiResponse};

pub struct AdminMiddleware;

impl AdminMiddleware {
    pub async fn handle(
        Extension(user): Extension<UserModel>,
        req: Request,
        next: Next,
    ) -> impl IntoResponse {
        if user.is_authorize_as_admin() {
            let status = StatusCode::FORBIDDEN;
            return ApiResponse::<bool>::error(Some(status.to_string()), Some(status))
                .into_response();
        }

        next.run(req).await.into_response()
    }
}
