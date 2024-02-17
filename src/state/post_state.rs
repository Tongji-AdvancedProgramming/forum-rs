use std::sync::Arc;

use crate::{
    config::{database::Db, meili::Meili, AppConfig},
    service::post_service::PostService,
};

#[derive(Clone)]
pub struct PostState {
    pub post_service: PostService,
}

impl PostState {
    pub fn new(db: &Arc<Db>, app_config: &Arc<AppConfig>, meili_client: &Arc<Meili>) -> Self {
        Self {
            post_service: PostService::new(db, app_config, meili_client),
        }
    }
}
