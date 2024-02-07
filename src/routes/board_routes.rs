use axum::{routing::get, Router};

use crate::state::board_state::BoardState;

pub fn routes() -> Router<BoardState> {
    use crate::handler::board_handler::*;

    Router::new().route("/", get(get_board_info))
}
