use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use either::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, utoipa::ToSchema)]
pub struct ApiResponse<T: Serialize = i32> {
    pub(crate) code: i32,
    pub(crate) message: String,
    pub(crate) data: Option<T>,
}

impl<T: Serialize> ApiResponse<T>
    where
        T: Serialize,
{
    pub(crate) fn send(data: Either<T, &str>) -> Self {
        match data {
            Either::Left(data) => ApiResponse { code: 10000, message: String::from("成功"), data: Some(data) },
            Either::Right(message) => ApiResponse { code: 5000, message: String::from(message), data: None }
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            Json(self)
        ).into_response()
    }
}
