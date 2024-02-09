use std::error::Error;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ApiResponse<T: Serialize = i32> {
    #[serde(skip)]
    status_code: StatusCode,

    pub(crate) code: i32,
    pub(crate) message: String,
    pub(crate) data: Option<T>,
}

impl<T: Serialize> ApiResponse<T>
where
    T: Serialize,
{
    pub(crate) fn send<E>(data: Result<T, E>) -> Self
    where
        E: Error,
    {
        match data {
            Ok(data) => Self::ok(data),
            Err(message) => Self::err(message),
        }
    }

    pub(crate) fn ok(data: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            code: 10000,
            message: String::from("成功"),
            data: Some(data),
        }
    }
}

impl ApiResponse {
    pub(crate) fn err<E>(error: E) -> Self
    where
        E: Error,
    {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            code: 5000,
            message: error.to_string(),
            data: None,
        }
    }

    pub(crate) fn err_with_code<E, U>(error: E, code: U) -> Self
    where
        E: Error,
        U: Into<StatusCode>,
    {
        Self {
            status_code: code.into(),
            code: 5000,
            message: error.to_string(),
            data: None,
        }
    }
}

impl<T: Serialize> From<T> for ApiResponse<T> {
    fn from(value: T) -> Self {
        Self::ok(value)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}
