use axum::{Json, response::IntoResponse};

use crate::models::auth_model::AuthModel;

pub struct AuthHandler;

impl AuthHandler {
    pub async fn sign_up(Json(payload): Json<AuthModel>) -> impl IntoResponse {}
}
