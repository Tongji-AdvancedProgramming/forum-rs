use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("ORM框架报数据库错误")]
    SeaOrmDatabaseError,
}

impl From<DbErr> for DbError {
    fn from(_value: DbErr) -> Self {
        DbError::SeaOrmDatabaseError
    }
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "系统内部错误").into_response()
    }
}
