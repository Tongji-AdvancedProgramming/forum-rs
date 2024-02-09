use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;

use thiserror::Error;

use crate::response::api_response::ApiResponse;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("ORM框架报数据库错误")]
    SeaOrmDatabaseError,
    #[error("Minio执行错误")]
    MinioError,
    #[error("{0}")]
    GeneralError(String),
}

impl From<DbErr> for ProcessError {
    fn from(_: DbErr) -> Self {
        ProcessError::SeaOrmDatabaseError
    }
}

use minio::s3::error::Error as MinioErr;
impl From<MinioErr> for ProcessError {
    fn from(_: MinioErr) -> Self {
        ProcessError::MinioError
    }
}

impl IntoResponse for ProcessError {
    fn into_response(self) -> Response {
        ApiResponse::err_with_code(self, StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
