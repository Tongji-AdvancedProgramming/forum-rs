use axum::extract::State;
use axum_login::AuthUser;

use crate::{
    dto::course_tree::CourseTree, entity::course, error::api_error::ApiError,
    response::api_response::ApiResponse, service::course_service::CourseServiceTrait,
    state::course_state::CourseState,
};

use super::AuthSession;

/// 当前用户拥有访问权的课程
///
/// 学生可访问已选课，助教和教师可访问所有课程
#[utoipa::path(
    get,
    path = "/course/my-course",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(Vec<(String, String)>))
    ),
)]
pub async fn get_my_courses(
    State(state): State<CourseState>,
    auth_session: AuthSession,
) -> Result<ApiResponse<Vec<(String, String)>>, ApiError> {
    let id = auth_session.user.unwrap().id();
    Ok(ApiResponse::send(Ok(state
        .course_service
        .get_user_courses(&id)
        .await?)))
}

/// 当前用户拥有访问权的课程详情
///
/// 学生可访问已选课，助教和教师可访问所有课程
#[utoipa::path(
    get,
    path = "/course/my-course/detail",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(Vec<course::Model>))
    ),
)]
pub async fn get_my_courses_detail(
    State(state): State<CourseState>,
    auth_session: AuthSession,
) -> Result<ApiResponse<Vec<course::Model>>, ApiError> {
    let id = auth_session.user.unwrap().id();
    Ok(ApiResponse::send(Ok(state
        .course_service
        .get_user_courses_detail(&id)
        .await?)))
}

/// 当前用户拥有访问权的课程代码
///
/// 学生可访问已选课，助教和教师可访问所有课程
#[utoipa::path(
    get,
    path = "/course/my-course-code",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(Vec<(String, String)>))
    ),
)]
pub async fn get_my_course_codes(
    State(state): State<CourseState>,
    auth_session: AuthSession,
) -> Result<ApiResponse<Vec<(String, String)>>, ApiError> {
    let id = auth_session.user.unwrap().id();
    Ok(ApiResponse::send(Ok(state
        .course_service
        .get_user_course_codes(&id)
        .await?)))
}

/// 获取课程树
///
/// 课程树包含了课程、班级、周次和作业等信息
#[utoipa::path(
    get,
    path = "/course/tree",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(CourseTree))
    ),
)]
pub async fn get_course_tree(
    State(state): State<CourseState>,
    auth_session: AuthSession,
) -> Result<ApiResponse<CourseTree>, ApiError> {
    let id = auth_session.user.unwrap().id();
    Ok(ApiResponse::send(Ok(state
        .course_service
        .get_user_courses_tree(&id)
        .await?)))
}
