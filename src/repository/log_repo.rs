use crate::config::database::{DatabaseTrait, Db};
use crate::entity::{log_login, log_post};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbErr};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LogRepository {
    db_conn: Arc<Db>,
}

impl LogRepository {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}

#[async_trait]
pub trait LogRepositoryTrait {
    type Error;

    async fn add_login(&self, log: log_login::Model) -> Result<(), Self::Error>;

    async fn add_post(&self, log: log_post::Model) -> Result<(), Self::Error>;
}

#[async_trait]
impl LogRepositoryTrait for LogRepository {
    type Error = DbErr;

    async fn add_login(&self, log: log_login::Model) -> Result<(), Self::Error> {
        log_login::ActiveModel {
            log_login_id: Default::default(),
            ..log_login::ActiveModel::from(log)
        }
        .save(self.db_conn.get_db())
        .await
        .map(|_| ())
    }

    async fn add_post(&self, log: log_post::Model) -> Result<(), Self::Error> {
        log_post::ActiveModel {
            log_post_id: Default::default(),
            ..log_post::ActiveModel::from(log)
        }
        .save(self.db_conn.get_db())
        .await
        .map(|_| ())
    }
}
