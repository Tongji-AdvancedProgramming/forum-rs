pub mod auth_handler;
pub mod board_handler;
pub mod course_handler;
pub mod homework_handler;
pub mod swagger_handler;
pub mod user_handler;

type AuthSession = axum_login::AuthSession<crate::service::auth_service::AuthBackend>;
