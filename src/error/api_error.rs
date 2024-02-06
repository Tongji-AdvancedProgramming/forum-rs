use crate::error::auth_error::AuthError;
use crate::error::limit_error::LimitError;
use crate::error::param_error::ParameterError;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use thiserror::Error;

use super::db_error::DbError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ParameterError(#[from] ParameterError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    LimitError(#[from] LimitError),
    #[error(transparent)]
    DbError(#[from] DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ParameterError(error) => error.into_response(),
            ApiError::AuthError(error) => error.into_response(),
            ApiError::LimitError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
        }
    }
}

impl From<DbErr> for ApiError {
    fn from(value: DbErr) -> Self {
        Self::DbError(DbError::SeaOrmDatabaseError)
    }
}
