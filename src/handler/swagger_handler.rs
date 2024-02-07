use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::user_handler::info,
        super::auth_handler::post::login,
        super::auth_handler::get::logout,
        super::board_handler::get_board_info,
    ),
    components(
        schemas(
            crate::response::api_response::ApiResponse,
            crate::entity::student::Model,
            crate::service::auth_service::Credentials,
            crate::dto::board::Board,
        )
    ),
    tags(
        (name = "User", description = "用户相关API"),
        (name = "Auth", description = "登录、验证相关API"),
        (name = "Board", description = "板块相关API"),
    )
)]
pub struct ApiDoc;
