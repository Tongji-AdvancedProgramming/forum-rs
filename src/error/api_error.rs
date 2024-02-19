use crate::error::auth_error::AuthError;
use crate::error::limit_error::LimitError;
use crate::error::param_error::ParameterError;
use axum::response::{IntoResponse, Response};
use log::error;
use sea_orm::DbErr;
use thiserror::Error;

use super::proc_error::ProcessError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ParameterError(#[from] ParameterError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    LimitError(#[from] LimitError),
    #[error(transparent)]
    ProcessError(#[from] ProcessError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ParameterError(error) => error.into_response(),
            ApiError::AuthError(error) => error.into_response(),
            ApiError::LimitError(error) => error.into_response(),
            ApiError::ProcessError(error) => error.into_response(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::ProcessError(ProcessError::SeaOrmDatabaseError(value))
    }
}

use minio::s3::error::Error as MinioErr;
impl From<MinioErr> for ApiError {
    fn from(value: MinioErr) -> Self {
        Self::ProcessError(ProcessError::MinioError(value))
    }
}
