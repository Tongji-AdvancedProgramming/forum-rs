use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::user_handler::info,
        super::auth_handler::post::login,
        super::auth_handler::get::logout,
    ),
    components(
        schemas(
            crate::response::api_response::ApiResponse,
            crate::entity::student::Student,
            crate::service::auth::Credentials,
        )
    ),
    tags(
        (name = "User", description = "用户相关API"),
        (name = "Auth", description = "登录、验证相关API"),
    )
)]
pub struct ApiDoc;
