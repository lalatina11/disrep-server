use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthModel {
    pub email: String,
    pub password: String,
}
