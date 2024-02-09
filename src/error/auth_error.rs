use crate::response::api_response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("用户名和密码错误")]
    WrongUsernameOrPassword,
    #[error("内部错误，验证失败")]
    AuthFailed,
    #[error("验证码不正确")]
    CaptchaWrong,
    #[error("未提供验证码")]
    CaptchaMissing,
    #[error("验证码生成失败")]
    CaptchaGenerateFailed,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AuthError::WrongUsernameOrPassword => StatusCode::UNAUTHORIZED,
            AuthError::CaptchaMissing | AuthError::CaptchaWrong => StatusCode::FORBIDDEN,
            AuthError::AuthFailed | AuthError::CaptchaGenerateFailed => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        ApiResponse::err_with_code(self, status_code).into_response()
    }
}
