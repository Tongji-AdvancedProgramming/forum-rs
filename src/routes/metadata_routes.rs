use axum::{routing::get, Router};

use crate::state::metadata_state::MetadataState;

pub fn routes() -> Router<MetadataState> {
    use crate::handler::metadata_handler as handler;

    Router::new().route("/tags", get(handler::get::tags))
}
