use reqwest::StatusCode;

#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub status: u16,
}

impl AuthError {
    pub fn internal() -> Self {
        Self {
            message: "An unexpected error occurred".to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }

    pub fn unauthorized(msg: Option<String>) -> Self {
        let status = StatusCode::UNAUTHORIZED;
        Self {
            message: msg.unwrap_or_else(|| status.to_string()),
            status: status.as_u16(),
        }
    }
}
