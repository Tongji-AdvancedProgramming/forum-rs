use axum::{routing::put, Router};

use crate::handler::post_handler as handler;
use crate::state::post_state::PostState;

pub fn routes() -> Router<PostState> {
    Router::new().route("/tag", put(handler::set_post_tags))
}
