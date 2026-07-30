use axum::{extract::Request, middleware::Next, response::IntoResponse};

pub struct LoggerMiddleware;

impl LoggerMiddleware {
    pub async fn handler(req: Request, next: Next) -> impl IntoResponse {
        let method = req.method().to_string();
        let uri = req.uri().to_string();
        let response = next.run(req).await;
        let status = response.status();
        println!("[{}] {} -> Status Code: {}", method, uri, status.as_u16());
        response.into_response()
    }
}
