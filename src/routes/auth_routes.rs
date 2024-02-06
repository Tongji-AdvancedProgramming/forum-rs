use crate::handler::auth_handler;
use crate::state::auth_state::AuthState;
use axum::routing::{get, post};
use axum::Router;

pub fn routes() -> Router<AuthState> {
    Router::new()
        .route("/login", post(auth_handler::post::login))
        .route("/login", get(auth_handler::get::login))
        .route("/logout", get(auth_handler::get::logout))
        .route("/captcha", get(auth_handler::get::captcha))
}
