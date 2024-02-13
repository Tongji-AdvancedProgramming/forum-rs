use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use sea_orm::{EntityTrait, QueryOrder};

use crate::{
    config::database::{DatabaseTrait, Db},
    entity::tag,
    error::api_error::ApiError,
};

#[async_trait]
pub trait MetadataServiceTrait {
    async fn get_tags(&self) -> Result<Vec<tag::Model>, ApiError>;
}

#[derive(Clone)]
pub struct MetadataService {
    db_conn: Arc<Db>,
}

static TAG_CACHE: Lazy<RwLock<(Vec<tag::Model>, DateTime<Local>)>> =
    Lazy::new(|| RwLock::new((vec![], Local::now())));

impl MetadataService {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}

#[async_trait]
impl MetadataServiceTrait for MetadataService {
    async fn get_tags(&self) -> Result<Vec<tag::Model>, ApiError> {
        let cache = TAG_CACHE.read().clone();
        if Local::now() > cache.1 {
            let tags = tag::Entity::find()
                .order_by_asc(tag::Column::TagFieldName)
                .all(self.db_conn.get_db())
                .await?;

            let mut guard = TAG_CACHE.write();
            guard.0 = tags;
            guard.1 = Local::now();
            return Ok(guard.0.clone());
        }

        Ok(cache.0.clone())
    }
}
