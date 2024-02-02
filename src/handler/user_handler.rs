use std::collections::HashMap;

use axum::extract::{Query, State};


use crate::entity::student::Student;
use crate::error::api_error::ApiError;
use crate::error::param_error::ParameterError;
use crate::response::api_response::ApiResponse;
use crate::state::user_state::UserState;

#[utoipa::path(
    get,
    path = "/user/info",
    tag = "User",
    responses(
        (status = 200, description = "查询学生成功", body = inline(Student)),
        (status = NOT_FOUND, description = "查询学生失败")
    ),
    params(
        ("id" = String, Query, description = "学生的ID"),
    ),
)]
pub async fn info(
    State(state): State<UserState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<ApiResponse<Student>, ApiError> {
    let id: Option<&String> = params.get("id");

    if id.is_none() {
        return Err(ApiError::ParameterError(ParameterError::MissingParameter(
            "id".to_string(),
        )));
    }

    match state.user_service.get_by_id(&id.unwrap()).await {
        Some(user) => Ok(ApiResponse::send(either::Left(user))),
        None => Ok(ApiResponse::send(either::Right("用户Id不存在"))),
    }
}
