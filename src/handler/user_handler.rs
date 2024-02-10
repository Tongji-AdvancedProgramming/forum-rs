use std::collections::HashMap;

use axum::extract::{Query, State};

use crate::entity::student;
use crate::error::api_error::ApiError;
use crate::error::param_error::ParameterError;
use crate::error::proc_error::ProcessError;
use crate::response::api_response::ApiResponse;
use crate::state::user_state::UserState;

#[utoipa::path(
    get,
    path = "/user/info",
    tag = "User",
    responses(
        (status = 200, description = "查询学生成功", body = inline(student::Model)),
        (status = NOT_FOUND, description = "查询学生失败")
    ),
    params(
        ("id" = String, Query, description = "学生的ID"),
    ),
)]
pub async fn info(
    State(state): State<UserState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<ApiResponse<student::Model>, ApiError> {
    let id: Option<&String> = params.get("id");

    if id.is_none() {
        return Err(ApiError::ParameterError(ParameterError::MissingParameter(
            "id".to_string(),
        )));
    }

    match state.user_service.get_by_id(&id.unwrap()).await {
        Some(user) => Ok(ApiResponse::ok(user)),
        None => Err(ApiError::ProcessError(ProcessError::GeneralError(
            "用户Id不存在".into(),
        ))),
    }
}
