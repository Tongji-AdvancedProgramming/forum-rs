use crate::config::database::{DatabaseTrait, Db};
use crate::entity::notification::{Column as Cols, Entity, Model as Notification};
use crate::error::api_error::ApiError;
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use std::sync::Arc;

#[derive(Clone)]
pub struct NotificationRepository {
    db_conn: Arc<Db>,
}

impl NotificationRepository {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn find_all_by_receiver_order_by_date_time_desc(
        &self,
        receiver: &str,
    ) -> Result<Vec<Notification>, ApiError> {
        Entity::find()
            .filter(Cols::NtfReceiver.eq(receiver))
            .order_by_desc(Cols::NtfDatetime)
            .all(self.db_conn.get_db())
            .await
            .map_err(Into::into)
    }

    pub async fn delete_all_by_date_time_before(
        &self,
        before: NaiveDateTime,
    ) -> Result<(), ApiError> {
        Entity::delete_many()
            .filter(Cols::NtfDatetime.lt(before))
            .exec(self.db_conn.get_db())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    pub async fn delete_all_by_date_time_before_and_read(
        &self,
        before: NaiveDateTime,
    ) -> Result<(), ApiError> {
        Entity::delete_many()
            .filter(Cols::NtfDatetime.lt(before))
            .filter(Cols::NtfReceiver.eq(1))
            .exec(self.db_conn.get_db())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    pub async fn find_all_by_receiver_and_read(
        &self,
        receiver: &str,
        read: bool,
    ) -> Result<Vec<Notification>, ApiError> {
        Entity::find()
            .filter(Cols::NtfReceiver.eq(receiver))
            .filter(Cols::NtfRead.eq(read))
            .all(self.db_conn.get_db())
            .await
            .map_err(Into::into)
    }

    pub async fn delete_all_by_receiver(&self, receiver: &str) -> Result<(), ApiError> {
        Entity::delete_many()
            .filter(Cols::NtfReceiver.eq(receiver))
            .exec(self.db_conn.get_db())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }
}
