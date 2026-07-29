use reqwest::StatusCode;
pub struct AuthError {
    pub message: String,
    pub status: StatusCode,
}

impl AuthError {
    pub fn internal() -> Self {
        Self {
            message: "An unexpected error occurred".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
