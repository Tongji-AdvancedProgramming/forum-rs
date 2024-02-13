use std::{collections::VecDeque, sync::Arc};

use async_trait::async_trait;
use meilisearch_sdk::Client;
use once_cell::sync::{Lazy, OnceCell};
use parking_lot::{lock_api::Mutex, Mutex};
use sea_orm::EntityTrait;
use tokio::time;

use crate::{
    config::{
        database::{DatabaseTrait, Db},
        AppConfig,
    },
    entity::post,
    error::proc_error::ProcessError,
};

#[async_trait]
pub trait SearchEngineServiceTrait {
    async fn add_post(&self, post_id: i32) -> Result<(), ProcessError>;
}

static ONCE_LOCK: OnceCell<()> = OnceCell::new();
static POSTID_QUEUE: Lazy<Mutex<VecDeque<i32>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

#[derive(Clone)]
pub struct SearchEngineService {
    db_conn: Arc<Db>,
    meili_client: Arc<Client>,
    app_config: Arc<AppConfig>,
}

impl SearchEngineService {
    pub fn new(meili_client: &Arc<Client>, db_conn: &Arc<Db>, app_config: &Arc<AppConfig>) -> Self {
        let _self = Self {
            meili_client: Arc::clone(meili_client),
            db_conn: Arc::clone(db_conn),
            app_config: Arc::clone(app_config),
        };

        // 维持单例状态
        if ONCE_LOCK.get().is_none() {
            ONCE_LOCK.set(()).unwrap();
            _self.run();
        }

        _self
    }

    pub fn run(&self) {
        let db = Arc::clone(&self.db_conn);
        let client = Arc::clone(&self.meili_client);
        let config = Arc::clone(&self.app_config);
        tokio::spawn(async move {
            let mut interval = time::interval(time::Duration::from_secs(30));
            loop {
                interval.tick().await;

                let queue = POSTID_QUEUE.get_mut().unwrap();
                while !queue.is_empty() {
                    let post_id = queue.pop_front().unwrap();
                    let mut post = post::Entity::find_by_id(post_id).one(db.get_db()).await;
                    if let Ok(Some(post)) = post {
                        if post.post_is_del == "1" {
                            client
                                .index(&config.meili.index.post)
                                .delete_document(post_id)
                                .await;
                            continue;
                        }
                    }
                }
            }
        });
    }
}

#[async_trait]
impl SearchEngineServiceTrait for SearchEngineService {
    async fn add_post(&self, post_id: i32) -> Result<(), ProcessError> {
        todo!()
    }
}
