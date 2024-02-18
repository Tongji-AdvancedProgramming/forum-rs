use std::sync::Arc;

use crate::{config::database::Db, service::notification_service::NotificationService};

#[derive(Clone)]
pub struct NotificationState {
    pub notification_service: NotificationService,
}

impl NotificationState {
    pub fn new(db: &Arc<Db>) -> Self {
        Self {
            notification_service: NotificationService::new(db),
        }
    }
}
