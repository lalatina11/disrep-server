use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::time::OffsetDateTime};

use crate::models::auth_model::AuthPayload;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl UserModel {
    pub fn to_payload(self, access_token: String) -> AuthPayload {
        AuthPayload {
            access_token,
            user: self,
        }
    }
}
