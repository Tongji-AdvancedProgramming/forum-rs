use crate::{entity::course, error::api_error::ApiError, response::api_response::ApiResponse};

use super::AuthSession;

/// 当前用户拥有访问权的课程
///
/// 学生可访问已选课，助教和教师可访问所有课程
#[utoipa::path(
    get,
    path = "/course/my-course",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(Vec<Vec<String>>))
    ),
)]
pub async fn get_my_courses(
    _auth_session: AuthSession,
) -> Result<ApiResponse<Vec<Vec<String>>>, ApiError> {
    unimplemented!()
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
    _auth_session: AuthSession,
) -> Result<ApiResponse<Vec<course::Model>>, ApiError> {
    unimplemented!()
}

/// 当前用户拥有访问权的课程代码
///
/// 学生可访问已选课，助教和教师可访问所有课程
#[utoipa::path(
    get,
    path = "/course/my-course-code",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(Vec<Vec<String>>))
    ),
)]
pub async fn get_my_course_codes(
    _auth_session: AuthSession,
) -> Result<ApiResponse<Vec<Vec<String>>>, ApiError> {
    unimplemented!()
}

/// 获取课程树
///
/// 课程树包含了课程、班级、周次和作业等信息
#[utoipa::path(
    get,
    path = "/course/tree",
    tag = "Course",
    responses(
        (status = 200, description = "获取课程成功", body = inline(course::Model))
    ),
)]
pub async fn get_course_tree(
    _auth_session: AuthSession,
) -> Result<ApiResponse<course::Model>, ApiError> {
    unimplemented!()
}
