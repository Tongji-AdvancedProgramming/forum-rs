use std::sync::Arc;

use async_trait::async_trait;
use forum_utils::html_cleaner::HtmlCleaner;
use once_cell::sync::OnceCell;
use sea_orm::EntityTrait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    config::{
        database::{DatabaseTrait, Db},
        meili::Meili,
        AppConfig,
    },
    entity::post,
    error::proc_error::ProcessError,
};

#[async_trait]
pub trait SearchEngineServiceTrait {
    async fn add_post(&self, post_id: i32) -> Result<(), ProcessError>;
}

static SERVICE_RUNNER: OnceCell<Arc<SearchEngineServiceRunner>> = OnceCell::new();

pub struct SearchEngineServiceRunner {
    db_conn: Arc<Db>,
    meili_client: Arc<Meili>,
    app_config: Arc<AppConfig>,

    sender: UnboundedSender<i32>,
}

impl SearchEngineServiceRunner {
    pub fn new(meili_client: &Arc<Meili>, db_conn: &Arc<Db>, app_config: &Arc<AppConfig>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        let _self = Self {
            meili_client: Arc::clone(meili_client),
            db_conn: Arc::clone(db_conn),
            app_config: Arc::clone(app_config),
            sender,
        };

        _self.run(receiver);

        _self
    }

    pub fn add_post(&self, id: i32) {
        self.sender.send(id).unwrap();
    }

    fn run(&self, mut receiver: UnboundedReceiver<i32>) {
        let db = Arc::clone(&self.db_conn);
        let meili_client = Arc::clone(&self.meili_client);
        let config = Arc::clone(&self.app_config);
        tokio::spawn(async move {
            loop {
                let post_id = receiver.recv().await;
                if post_id.is_none() {
                    // Sender已经被全部销毁，我也销毁
                    break;
                }
                let post_id = post_id.unwrap();

                let mut post = post::Entity::find_by_id(post_id).one(db.get_db()).await;
                if let Ok(Some(post)) = post {
                    if post.post_is_del == "1" {
                        let _ = meili_client
                            .get_client()
                            .index(&config.meili.index.post)
                            .delete_document(post_id)
                            .await;
                        continue;
                    }

                    let post = post::Model {
                        post_content: Some(HtmlCleaner::html_to_text(
                            &post.post_content.unwrap_or_default(),
                        )),
                        ..post
                    };

                    let _ = meili_client
                        .get_client()
                        .index(&config.meili.index.post)
                        .add_documents(&[post], None)
                        .await;
                } // 做一些事情
            } // 业务代码
        });
    }
}

#[derive(Clone)]
pub struct SearchEngineService {
    runner: Arc<SearchEngineServiceRunner>,
}

impl SearchEngineService {
    pub fn new(meili_client: &Arc<Meili>, db_conn: &Arc<Db>, app_config: &Arc<AppConfig>) -> Self {
        let runner = SERVICE_RUNNER
            .get_or_init(|| {
                Arc::new(SearchEngineServiceRunner::new(
                    meili_client,
                    db_conn,
                    app_config,
                ))
            })
            .clone();
        Self { runner }
    }
}

#[async_trait]
impl SearchEngineServiceTrait for SearchEngineService {
    async fn add_post(&self, post_id: i32) -> Result<(), ProcessError> {
        self.runner.add_post(post_id);
        Ok(())
    }
}
