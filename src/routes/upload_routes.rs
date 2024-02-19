use axum::{routing::post, Router};

use crate::{handler::upload_handler as handler, state::upload_state::UploadState};

pub fn routes() -> Router<UploadState> {
    Router::new().route("/images", post(handler::add_image))
}
