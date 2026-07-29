use serde::{Deserialize, Serialize};

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
