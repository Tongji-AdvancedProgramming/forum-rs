use crate::config::redis::RedisTrait;
use crate::error::limit_error::LimitError;
use crate::state::limit_state::LimitState;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use axum_client_ip::SecureClientIp;
use axum_macros::debug_handler;
use chrono::{Duration, Local, NaiveDateTime};
use fred::interfaces::{KeysInterface, ListInterface, RedisResult};
use fred::prelude::Expiration;
use log::warn;

pub async fn rate_limit_middleware(
    State(state): State<LimitState>,
    SecureClientIp(ip_addr): SecureClientIp,
    request: Request,
    next: Next,
) -> Result<Response, LimitError> {
    let redis = state.redis.get_pool();
    let core_fun = || async move {
        let key = format!("limit-{}", ip_addr);
        let lt: Option<i32> = redis.get(&key).await?;
        if lt.is_some() && lt.unwrap() >= 10 {
            return Ok(false);
        } else {
            redis
                .set(
                    &key,
                    lt.unwrap_or(0) + 1,
                    Some(Expiration::EX(60)),
                    None,
                    false,
                )
                .await?;
        }

        Ok(true)
    };

    let result: RedisResult<bool> = core_fun().await;
    if result.is_err() {
        warn!("限流异常： {}", result.err().unwrap());
        Ok(next.run(request).await)
    } else if result.unwrap() == false {
        Err(LimitError::TooManyRequests)
    } else {
        Ok(next.run(request).await)
    }
}
