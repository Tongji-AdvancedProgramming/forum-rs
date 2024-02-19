use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use log::error;
use minio::s3::error::Error as MinioErr;
use sea_orm::DbErr;

use thiserror::Error;

use crate::response::api_response::ApiResponse;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("ORM框架报数据库错误:{0}")]
    SeaOrmDatabaseError(DbErr),
    #[error("Minio执行错误:{0}")]
    MinioError(MinioErr),
    #[error("{0}")]
    GeneralError(&'static str),
}

impl From<DbErr> for ProcessError {
    fn from(value: DbErr) -> Self {
        ProcessError::SeaOrmDatabaseError(value)
    }
}

impl From<MinioErr> for ProcessError {
    fn from(value: MinioErr) -> Self {
        ProcessError::MinioError(value)
    }
}

impl IntoResponse for ProcessError {
    fn into_response(self) -> Response {
        ApiResponse::err_with_code(self, StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
