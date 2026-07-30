use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct SignUpUserName {
    #[validate(length(min = 3, max = 128))]
    display_name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignUpPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
    pub data: SignUpUserName,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct SignInPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}
