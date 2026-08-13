use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::{
    constants::ROLE_LIST,
    models::user_model::UserModel,
    utils::responses::auth_responses::{AppMetadata, GetUserIdentity, GetUserMetadata},
};

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SignUpAdditionalData {
    #[validate(length(min = 3, max = 128, message = "User name must be 3-128 characters"))]
    pub display_name: String,
    #[validate(custom(function = "validate_user_role"))]
    pub role: Option<String>,
}

fn validate_user_role(role: &str) -> Result<(), ValidationError> {
    if !ROLE_LIST.contains(&role) {
        return Err(ValidationError::new("Invalid user role"));
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct SignUpPayload {
    #[validate(email(message = "Please enter a valid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 32, message = "Password must between 8-32 characters"))]
    pub password: String,
    pub data: SignUpAdditionalData,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignInPayload {
    #[validate(email(message = "Please enter a valid email"))]
    pub email: String,
    #[validate(length(min = 8, max = 32, message = "Password must between 8-32 characters"))]
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
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub token: AuthToken,
    pub user: UserModel,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefreshTokenPayload {
    pub refresh_token: String,
}
