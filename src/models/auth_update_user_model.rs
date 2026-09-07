use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    models::user_model::{NewUser, UserModel},
    service::user_service::UserService,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalData {
    pub avatar: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct AuthUpdateUserPayload {
    pub password: Option<String>,
    pub data: AdditionalData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProfilePayload {
    #[validate(length(min = 3, max = 128, message = "User name must be 3-128 characters"))]
    pub display_name: String,
    #[validate(length(min = 3, max = 128, message = "Invalid avatar URL"))]
    pub avatar: String,
}

impl UpdateProfilePayload {
    pub fn to_update_user_payload(self) -> AuthUpdateUserPayload {
        AuthUpdateUserPayload {
            password: None,
            data: AdditionalData {
                avatar: Some(self.avatar),
                display_name: Some(self.display_name),
            },
        }
    }
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

impl UpdateUserResult {
    pub fn to_new_user_payload(self, current_user: UserModel) -> NewUser {
        let display_name = self
            .user_metadata
            .display_name
            .unwrap_or(current_user.display_name);
        let new_avatar = UserService::generate_avatar(&display_name);
        let avatar: String = if let Some(avatar) = self.user_metadata.avatar {
            if avatar != "" { avatar } else { new_avatar }
        } else {
            current_user.avatar.unwrap_or(new_avatar)
        };
        NewUser {
            id: self.id,
            email: self.user_metadata.email,
            display_name,
            role: current_user.role,
            avatar: Some(avatar),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetadata {
    pub display_name: Option<String>,
    pub avatar: Option<String>,
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
