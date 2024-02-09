use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("ORM框架报数据库错误")]
    SeaOrmDatabaseError,
    #[error("Minio执行错误")]
    MinioError,
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
        (StatusCode::INTERNAL_SERVER_ERROR, "系统内部错误").into_response()
    }
}
