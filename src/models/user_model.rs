use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{constants::ADMIN_ROLES, models::auth_model::AuthPayload};

#[derive(Clone, Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub avatar: Option<String>,
    pub avatar_storage_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserModel {
    pub fn is_authorize_as_admin(&self) -> bool {
        ADMIN_ROLES.contains(&self.role.as_str())
    }
}

impl UserModel {
    pub fn to_payload(self, access_token: String) -> AuthPayload {
        AuthPayload {
            access_token,
            user: self,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Validate)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub avatar: Option<String>,
}
