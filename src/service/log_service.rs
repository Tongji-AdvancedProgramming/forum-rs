use crate::config::database::Db;
use crate::repository::log_repo::{LogRepository, LogRepositoryTrait};
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime, Utc};
use log::{debug, info};
use sea_orm::prelude::DateTime;
use std::net::IpAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct LogService {
    log_repo: Arc<LogRepository>,
}

impl LogService {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            log_repo: Arc::new(LogRepository::new(db_conn)),
        }
    }
}

#[async_trait]
pub trait LogServiceTrait {
    async fn log_login(&self, stu_no: &str, ip_addr: &IpAddr, user_agent: &str, comment: &str);

    async fn log_post(&self, post_id: i32, stu_no: &str, ip_addr: &IpAddr, comment: &str);
}

#[async_trait]
impl LogServiceTrait for LogService {
    async fn log_login(&self, stu_no: &str, ip_addr: &IpAddr, user_agent: &str, comment: &str) {
        use crate::entity::log_login::Model as LogLogin;

        let log = LogLogin {
            log_login_id: 0,
            log_login_no: stu_no.into(),
            log_login_ipaddr: ip_addr.to_string(),
            log_login_date: Local::now().naive_local(),
            log_login_useragent: user_agent.into(),
            log_login_comment: comment.into(),
        };

        let log_repo = Arc::clone(&self.log_repo);

        tokio::spawn(async move {
            let res = log_repo.add_login(log).await;
            debug!("登录日志异步记录完成，结果是：{:?}", res)
        });
    }

    async fn log_post(&self, post_id: i32, stu_no: &str, ip_addr: &IpAddr, comment: &str) {
        use crate::entity::log_post::Model as LogPost;

        let log = LogPost {
            log_post_id: 0,
            log_post_post_id: post_id,
            log_post_op_no: stu_no.into(),
            log_post_ipaddr: ip_addr.to_string(),
            log_post_date: Local::now().naive_local(),
            log_post_comment: comment.into(),
        };

        let log_repo = Arc::clone(&self.log_repo);

        tokio::spawn(async move { log_repo.add_post(log).await });
    }
}
