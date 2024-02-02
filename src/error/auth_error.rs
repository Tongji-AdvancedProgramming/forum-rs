use crate::response::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("用户名和密码错误")]
    WrongUsernameOrPassword,
    #[error("内部错误，验证失败")]
    AuthFailed,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AuthError::WrongUsernameOrPassword => (StatusCode::UNAUTHORIZED, 4000),
            AuthError::AuthFailed => (StatusCode::INTERNAL_SERVER_ERROR, 5000),
        };

        let response: ApiResponse = ApiResponse {
            code: status_code.1,
            message: self.to_string(),
            data: None,
        };

        (status_code.0, Json(response)).into_response()
    }
}
