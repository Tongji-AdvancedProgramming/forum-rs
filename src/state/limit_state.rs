use crate::config::redis::Redis;
use std::sync::Arc;

#[derive(Clone)]
pub struct LimitState {
    pub redis: Arc<Redis>,
}

impl LimitState {
    pub fn new(redis: &Arc<Redis>) -> Self {
        Self {
            redis: Arc::clone(redis),
        }
    }
}
