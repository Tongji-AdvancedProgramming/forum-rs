use axum::response::{IntoResponse, Response};
use thiserror::Error;
use crate::error::param_error::ParameterError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ParameterError(#[from] ParameterError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ParameterError(error) => error.into_response(),
        }
    }
}
