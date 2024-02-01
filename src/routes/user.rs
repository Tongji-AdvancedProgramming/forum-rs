use axum::Router;
use axum::routing::get;
use crate::handler::user_handler;
use crate::state::user_state::UserState;

pub fn routes() -> Router<UserState> {
    let router = Router::new().route("/info", get(user_handler::info));
    return router;
}
