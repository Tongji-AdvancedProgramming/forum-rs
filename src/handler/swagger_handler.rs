use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
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
        super::notification_handler::get_my_notifications,
        super::notification_handler::read_my_notifications,
        super::notification_handler::read_all_my_notifications,
        super::notification_handler::delete_all_my_notifications,
        super::post_handler::set_post_tags,
        super::post_handler::set_post_priority,
        super::post_handler::add_post,
        super::post_handler::add_reply,
        super::post_handler::edit_post,
        super::post_handler::delete_posts,
        super::post_handler::list_posts,
        super::post_handler::get_posts,
        super::post_handler::get_post_parent,
        super::upload_handler::add_image,
        super::user_handler::get_me,
        super::user_handler::get_my_info,
        super::user_handler::set_nickname,
        super::user_handler::set_signature,
        super::user_handler::get_student_short_info,
        super::user_handler::put_avatar,
        super::user_handler::put_card_background,
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
        (name = "Auth", description = "登录、验证相关API"),
        (name = "Board", description = "板块相关API"),
        (name = "Course", description = "课程相关API"),
        (name = "Homework", description = "作业相关API"),
        (name = "Metadata", description = "元数据相关API"),
        (name = "Notification", description = "通知相关API"),
        (name = "Post", description = "帖子相关API"),
        (name = "Upload", description = "上传相关API"),
        (name = "User", description = "用户相关API"),
    )
)]
pub struct ApiDoc;
