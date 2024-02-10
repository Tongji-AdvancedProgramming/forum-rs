use axum::{
    routing::{get, post},
    Router,
};
use axum_login::permission_required;

use crate::{
    config::permission::Permission, service::auth_service::AuthBackend,
    state::homework_state::HomeworkState,
};

pub fn routes() -> Router<HomeworkState> {
    use crate::handler::homework_handler as handler;

    Router::new()
        .route("/uploaded", post(handler::post::homework_uploaded))
        .route_layer(permission_required!(AuthBackend, Permission::ADMIN))
        .route("/uploaded", get(handler::get::homework_uploaded))
        .route("/", get(handler::get::homework))
}
