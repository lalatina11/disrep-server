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
    pub display_name: Option<String>,
    #[validate(length(min = 3, max = 2048, message = "Invalid avatar URL"))]
    pub avatar: Option<String>,
    pub data: Option<AdditionalData>,
}

impl UpdateProfilePayload {
    pub fn get_display_name(&self) -> Option<String> {
        self.display_name
            .clone()
            .or_else(|| self.data.as_ref()?.display_name.clone())
    }

    pub fn get_avatar(&self) -> Option<String> {
        self.avatar
            .clone()
            .or_else(|| self.data.as_ref()?.avatar.clone())
    }

    pub fn to_supabase_payload(&self, current_user: &UserModel) -> AuthUpdateUserPayload {
        let display_name = self
            .get_display_name()
            .unwrap_or_else(|| current_user.display_name.clone());
        let avatar = self.get_avatar().or_else(|| current_user.avatar.clone());

        AuthUpdateUserPayload {
            password: None,
            data: AdditionalData {
                avatar,
                display_name: Some(display_name),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserResult {
    pub id: uuid::Uuid,
    pub aud: Option<String>,
    pub role: Option<String>,
    pub email: String,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub phone: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_anonymous: Option<bool>,
    pub app_metadata: Option<AppMetadata>,
    pub user_metadata: Option<UserMetadata>,
    pub identities: Option<Vec<Identity>>,
}

impl UpdateUserResult {
    pub fn to_new_user_payload(self, current_user: UserModel) -> NewUser {
        let display_name = self
            .user_metadata
            .as_ref()
            .and_then(|m| m.display_name.clone())
            .unwrap_or(current_user.display_name);
        let new_avatar = UserService::generate_avatar(&display_name);
        let avatar: String = if let Some(ref m) = self.user_metadata {
            if let Some(ref av) = m.avatar {
                if !av.is_empty() {
                    av.clone()
                } else {
                    new_avatar
                }
            } else {
                current_user.avatar.unwrap_or(new_avatar)
            }
        } else {
            current_user.avatar.unwrap_or(new_avatar)
        };
        NewUser {
            id: self.id,
            email: self.email,
            display_name,
            role: current_user.role,
            avatar: Some(avatar),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppMetadata {
    pub provider: Option<String>,
    pub providers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserMetadata {
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub role: Option<String>,
    pub sub: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityData {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub phone_verified: Option<bool>,
    pub role: Option<String>,
    pub sub: Option<uuid::Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Identity {
    pub identity_id: uuid::Uuid,
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub identity_data: Option<IdentityData>,
    pub provider: Option<String>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdatePasswordPayload {
    #[validate(length(
        min = 8,
        max = 32,
        message = "Current Password must between 8-32 characters"
    ))]
    pub current_password: String,
    #[validate(length(
        min = 8,
        max = 32,
        message = "New Password must between 8-32 characters"
    ))]
    pub new_password: String,
}
