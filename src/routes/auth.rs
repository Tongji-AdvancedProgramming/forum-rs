use crate::handler::auth_handler;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router<()> {
    Router::new()
        .route("/login", post(auth_handler::post::login))
        .route("/login", get(auth_handler::get::login))
        .route("/logout", get(auth_handler::get::logout))
}
