use serde::{Deserialize, Serialize};

use crate::traits::can_validate::CanValidate;

#[derive(Serialize, Deserialize)]
pub struct SignUpUserName {
    display_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpPayload {
    pub email: String,
    pub password: String,
    pub data: SignUpUserName,
}

#[derive(Serialize, Deserialize)]
pub struct SignInPayload {
    pub email: String,
    pub password: String,
}

impl CanValidate for SignUpPayload {
    fn validate(self) -> Result<Self, String> {
        if self.email == "" {
            return Err("email is required".to_string());
        }
        if self.email.trim() == "" {
            return Err("email is required".to_string());
        }
        if self.password.trim() == "" {
            return Err("email is required".to_string());
        }
        Ok(self)
    }
}
