use crate::config::database::{DatabaseTrait, Db};
use crate::entity::notification;
use crate::entity::notification::{Column, Entity, Model as Notification};
use crate::error::api_error::ApiError;
use crate::error::param_error::ParameterError;
use crate::repository::notification_repo::NotificationRepository;
use async_trait::async_trait;
use chrono::{Days, Local};
use once_cell::sync::OnceCell;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, NotSet, QueryFilter};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

#[async_trait]
pub trait NotificationServiceTrait {
    /// 发送通知
    async fn send_notification(&self, notification: Notification) -> Result<(), ApiError>;

    /// 用户获取通知
    async fn get_notifications(&self, user_id: &str) -> Result<Vec<Notification>, ApiError>;

    /// 用户已读通知
    async fn user_read_notification(&self, ntf_id: u64, user_id: &str) -> Result<(), ApiError>;

    /// 用户已读所有通知
    async fn user_read_all_notification(&self, user_id: &str) -> Result<(), ApiError>;

    /// 用户删除所有通知
    async fn user_delete_all_notification(&self, user_id: &str) -> Result<(), ApiError>;
}

static SERVICE_RUNNER: OnceCell<Arc<NotificationServiceRunner>> = OnceCell::new();

pub struct NotificationServiceRunner {
    notification_repository: NotificationRepository,
}

impl NotificationServiceRunner {
    pub fn init(db_conn: &Arc<Db>) {
        if SERVICE_RUNNER.get().is_none() {
            let runner = Arc::new(NotificationServiceRunner {
                notification_repository: NotificationRepository::new(db_conn),
            });
            if SERVICE_RUNNER.set(runner.clone()).is_ok() {
                runner.run();
            }
        }
    }

    fn run(&self) {
        let repo = self.notification_repository.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60 * 60));

            {
                let _ = repo
                    .delete_all_by_date_time_before(Local::now().naive_local() - Days::new(7))
                    .await;
                let _ = repo
                    .delete_all_by_date_time_before_and_read(
                        Local::now().naive_local() - Days::new(1),
                    )
                    .await;
            }

            interval.tick().await;
        });
    }
}

#[derive(Clone)]
pub struct NotificationService {
    notification_repository: NotificationRepository,
    db_conn: Arc<Db>,
}

impl NotificationService {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        NotificationServiceRunner::init(db_conn);
        Self {
            notification_repository: NotificationRepository::new(db_conn),
            db_conn: db_conn.clone(),
        }
    }
}

#[async_trait]
impl NotificationServiceTrait for NotificationService {
    async fn send_notification(&self, notification: Notification) -> Result<(), ApiError> {
        let mut notification = notification;
        notification.ntf_datetime = Local::now().naive_local();
        notification.ntf_read = false;

        let notification = notification::ActiveModel {
            ntf_id: NotSet,
            ..notification.into_active_model()
        };
        notification.save(self.db_conn.get_db()).await?;

        Ok(())
    }

    async fn get_notifications(&self, user_id: &str) -> Result<Vec<Notification>, ApiError> {
        self.notification_repository
            .find_all_by_receiver_order_by_date_time_desc(user_id)
            .await
    }

    async fn user_read_notification(&self, ntf_id: u64, user_id: &str) -> Result<(), ApiError> {
        if let Some(ntf) = Entity::find_by_id(ntf_id)
            .one(self.db_conn.get_db())
            .await?
        {
            if ntf.ntf_receiver == user_id {
                let ntf = notification::ActiveModel {
                    ntf_read: Set(true),
                    ..ntf.into_active_model()
                };
                ntf.save(self.db_conn.get_db()).await?;
                return Ok(());
            }
        }
        Err(ParameterError::InvalidParameter("无效的通知id").into())
    }

    async fn user_read_all_notification(&self, user_id: &str) -> Result<(), ApiError> {
        Entity::update_many()
            .col_expr(Column::NtfRead, Expr::value(true))
            .filter(Column::NtfReceiver.eq(user_id))
            .filter(Column::NtfRead.eq(false))
            .exec(self.db_conn.get_db())
            .await
            .map(|_| ())
            .map_err(Into::into)
    }

    async fn user_delete_all_notification(&self, user_id: &str) -> Result<(), ApiError> {
        self.notification_repository
            .delete_all_by_receiver(user_id)
            .await
    }
}
