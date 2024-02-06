use crate::response::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParameterError {
    #[error("未传入必须的参数：{0}")]
    MissingParameter(String),

    #[allow(dead_code)]
    #[error("参数值无效：{0}")]
    InvalidParameter(String),
}

impl IntoResponse for ParameterError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ParameterError::MissingParameter(_) | ParameterError::InvalidParameter(_) => {
                (StatusCode::BAD_REQUEST, 4000)
            }
        };

        let response: ApiResponse = ApiResponse {
            code: status_code.1,
            message: self.to_string(),
            data: None,
        };

        (status_code.0, Json(response)).into_response()
    }
}
