use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::user_handler::info,
        super::auth_handler::post::login,
        super::auth_handler::get::logout,
        super::board_handler::get_board_info,
        super::course_handler::get_my_courses,
        super::course_handler::get_my_courses_detail,
        super::course_handler::get_my_course_codes,
        super::homework_handler::get::homework,
        super::homework_handler::get::homework_uploaded,
        super::homework_handler::post::homework_uploaded,
        super::metadata_handler::get::tags,
    ),
    components(
        schemas(
            crate::response::api_response::ApiResponse,
            crate::entity::student::Model,
            crate::entity::course::Model,
            crate::entity::homework::Model,
            crate::entity::homework_uploaded::Model,
            crate::service::auth_service::Credentials,
            crate::dto::board::Board,
            crate::dto::course_tree::CourseTree,
        )
    ),
    tags(
        (name = "User", description = "用户相关API"),
        (name = "Auth", description = "登录、验证相关API"),
        (name = "Board", description = "板块相关API"),
        (name = "Course", description = "课程相关API"),
        (name = "Homework", description = "作业相关API"),
        (name = "Metadata", description = "元数据相关API"),
    )
)]
pub struct ApiDoc;
