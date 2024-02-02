use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::user_handler::info,

    ),
    components(
        schemas(
            crate::entity::student::Student,
        )
    ),
    tags(
        (name = "User", description = "用户相关API")
    )
)]
pub struct ApiDoc;
