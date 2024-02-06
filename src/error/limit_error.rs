use crate::response::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LimitError {
    #[error("请求过于频繁，请于一分钟后再试")]
    TooManyRequests,
}

impl IntoResponse for LimitError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::TooManyRequests => (StatusCode::TOO_MANY_REQUESTS, 4003),
        };

        let response: ApiResponse = ApiResponse {
            code: status_code.1,
            message: self.to_string(),
            data: None,
        };

        (status_code.0, Json(response)).into_response()
    }
}
