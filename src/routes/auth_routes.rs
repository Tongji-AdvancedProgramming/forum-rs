use crate::handler::auth_handler;
use crate::middleware::rate_limit::rate_limit_middleware;
use crate::state::auth_state::AuthState;
use crate::state::limit_state::LimitState;
use axum::routing::{get, post};
use axum::{middleware, Router};

pub fn routes(limit_state: LimitState) -> Router<AuthState> {
    Router::new()
        .route("/login", post(auth_handler::post::login))
        .route("/login", get(auth_handler::get::login))
        .route("/logout", get(auth_handler::get::logout))
        .route("/captcha", get(auth_handler::get::captcha))
        .route_layer(middleware::from_fn_with_state(
            limit_state,
            rate_limit_middleware,
        ))
}
