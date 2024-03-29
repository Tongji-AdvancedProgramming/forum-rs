use crate::response::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParameterError {
    #[allow(dead_code)]
    #[error("参数值无效：{0}")]
    InvalidParameter(&'static str),
}

impl IntoResponse for ParameterError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ParameterError::InvalidParameter(_) => StatusCode::BAD_REQUEST,
        };

        ApiResponse::err_with_code(self, status_code).into_response()
    }
}
