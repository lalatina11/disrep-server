use axum::body::Bytes;

#[derive(Debug, Clone)]
pub struct FileFormData {
    pub name: String,
    pub bytes: Option<Bytes>,
    pub content_type: Option<String>,
}
