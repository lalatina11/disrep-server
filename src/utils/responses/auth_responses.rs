use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignUpAndInSuccessResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub expires_at: i64,
    pub refresh_token: String,
    pub user: User,
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
pub struct SignUpErrorResponse {
    pub code: u16,
    pub error_code: String,
    pub msg: String,
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
