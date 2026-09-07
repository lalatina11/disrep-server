use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalData {
    pub avatar: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct AuthUpdateUserPayload {
    pub email: Option<String>,
    pub password: Option<String>,
    pub data: AdditionalData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserResult {
    pub id: uuid::Uuid,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub email_confirmed_at: DateTime<Utc>,
    pub phone: String,
    pub confirmed_at: DateTime<Utc>,
    pub last_sign_in_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_anonymous: bool,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub identities: Vec<Identity>,
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
    pub role: String,
    pub sub: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityData {
    pub display_name: String,
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub role: String,
    pub sub: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Identity {
    pub identity_id: uuid::Uuid,
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
}
