use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::user_model::UserModel,
    utils::responses::auth_responses::{AppMetadata, GetUserIdentity, GetUserMetadata},
};

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SignUpAdditionalData {
    #[validate(length(min = 3, max = 128))]
    pub display_name: String,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SignUpPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    pub data: SignUpAdditionalData,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignInPayload {
    #[validate(email(message = "Please use a valid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 32, message = "must between 8-32 characters"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPayload {
    pub id: String,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub email_confirmed_at: String,
    pub phone: String,
    pub confirmed_at: String,
    pub last_sign_in_at: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: GetUserMetadata,
    pub identities: Vec<GetUserIdentity>,
    pub created_at: String,
    pub updated_at: String,
    pub is_anonymous: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub access_token: String,
    pub user: UserModel,
}
