use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::ServiceError,
    models::user_model::{NewUser, UserModel},
    service::user_service::UserService,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpAndInSuccessResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub expires_at: i64,
    pub refresh_token: String,
    pub user: User,
}

impl SignUpAndInSuccessResponse {
    pub async fn create_user(&self) -> Result<UserModel, ServiceError> {
        UserService::create_user(NewUser {
            display_name: self.user.user_metadata.display_name.clone(),
            email: self.user.email.clone(),
            id: Uuid::from_str(&self.user.id).unwrap_or(uuid::Uuid::new_v4()),
            role: self.user.user_metadata.role.clone(),
            avatar: None,
        })
        .await
    }

    pub async fn check_existing_user(&self) -> Result<UserModel, ServiceError> {
        UserService::get_user_by_id(Uuid::from_str(&self.user.id).unwrap_or(Uuid::new_v4())).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub email_confirmed_at: String,
    pub phone: String,
    pub last_sign_in_at: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub identities: Vec<Identity>,
    pub created_at: String,
    pub updated_at: String,
    pub is_anonymous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetadata {
    pub display_name: String,
    pub email: String,
    pub role: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub identity_id: String,
    pub id: String,
    pub user_id: String,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityData {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSuccessResponse {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserMetadata {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserIdentity {
    pub identity_id: String,
    pub id: String,
    pub user_id: String,
    pub identity_data: GetUserIdentityData,
    pub provider: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserIdentityData {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}
