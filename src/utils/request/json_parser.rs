use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection},
    response::{IntoResponse, Response as AxumResponse},
};
use reqwest::StatusCode;
use serde::Serialize;

use crate::utils::responses::api_responses::ApiResponse;

pub struct JsonParser<T>(pub T);

impl<S, T> FromRequest<S> for JsonParser<T>
where
    T: Serialize,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AxumResponse;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let parsed_json = Json::<T>::from_request(req, state).await;
        match parsed_json {
            Err(rejection) => {
                let body = ApiResponse::<T>::error(
                    Some(rejection.body_text()),
                    Some(StatusCode::UNPROCESSABLE_ENTITY),
                );
                Err((rejection.status(), body).into_response())
            }
            Ok(Json(v)) => Ok(JsonParser(v)),
        }
    }
}
