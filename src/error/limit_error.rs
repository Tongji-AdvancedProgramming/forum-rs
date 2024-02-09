use crate::response::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LimitError {
    #[error("请求过于频繁，请于一分钟后再试")]
    TooManyRequests,
}

impl IntoResponse for LimitError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
        };

        ApiResponse::err_with_code(self, status_code).into_response()
    }
}
