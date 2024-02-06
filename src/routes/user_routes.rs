use crate::config::permission::Permission;
use crate::handler::user_handler;
use crate::service::auth_service::AuthBackend;
use crate::state::user_state::UserState;
use axum::routing::get;
use axum::Router;
use axum_login::permission_required;

pub fn routes() -> Router<UserState> {
    let router = Router::new()
        .route("/info", get(user_handler::info))
        .route_layer(permission_required!(AuthBackend, Permission::TA));
    return router;
}
