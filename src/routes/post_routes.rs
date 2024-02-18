use axum::routing::{delete, get, post, put};
use axum::Router;
use axum_login::permission_required;

use crate::config::permission::Permission;
use crate::handler::post_handler as handler;
use crate::service::auth_service::AuthBackend;
use crate::state::post_state::PostState;

pub fn routes() -> Router<PostState> {
    let ta_router = Router::new()
        .route("/tag", put(handler::set_post_tags))
        .route("/priority", put(handler::set_post_priority))
        .route_layer(permission_required!(AuthBackend, Permission::TA));

    Router::new()
        .merge(ta_router)
        .route("/", post(handler::add_post))
        .route("/reply", post(handler::add_reply))
        .route("/", put(handler::edit_post))
        .route("/", delete(handler::delete_posts))
        .route("/list", get(handler::list_posts))
        .route("/", get(handler::get_posts))
        .route("/parent", get(handler::get_post_parent))
}
