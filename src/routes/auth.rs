use crate::state::auth_state::AuthState;
use axum::Router;

pub fn routes() -> Router<AuthState> {
    Router::new()
}
