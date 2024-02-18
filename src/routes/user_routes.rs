use crate::handler::user_handler as handler;
use crate::state::user_state::UserState;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router<UserState> {
    Router::new()
        .route("/", get(handler::get_me))
        .route("/info", get(handler::get_my_info))
        .route("/nickName", post(handler::set_nickname))
        .route("/signature", post(handler::set_signature))
        .route("/shortInfo", get(handler::get_student_short_info))
        .route("/avatar", post(handler::put_avatar))
        .route("/cardBackground", post(handler::put_card_background))
}
