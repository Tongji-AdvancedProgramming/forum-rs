use crate::error::auth_error::AuthError;
use crate::error::limit_error::LimitError;
use crate::error::param_error::ParameterError;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ParameterError(#[from] ParameterError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    LimitError(#[from] LimitError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ParameterError(error) => error.into_response(),
            ApiError::AuthError(error) => error.into_response(),
            ApiError::LimitError(error) => error.into_response(),
        }
    }
}
