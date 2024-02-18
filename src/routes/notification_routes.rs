use axum::routing::{delete, get, post};
use axum::Router;

use crate::handler::notification_handler as handler;
use crate::state::notification_state::NotificationState;

pub fn routes() -> Router<NotificationState> {
    Router::new()
        .route("/", get(handler::get_my_notifications))
        .route("/read", post(handler::read_my_notifications))
        .route("/readAll", post(handler::read_all_my_notifications))
        .route("/all", delete(handler::delete_all_my_notifications))
}
