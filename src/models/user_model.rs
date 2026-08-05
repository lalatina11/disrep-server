use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::auth_model::AuthPayload;

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
    pub fn to_payload(self, access_token: String) -> AuthPayload {
        AuthPayload {
            access_token,
            user: self,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub avatar: Option<String>,
}
