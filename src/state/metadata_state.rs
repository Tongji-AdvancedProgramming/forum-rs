use std::sync::Arc;

use crate::{config::database::Db, service::metadata_service::MetadataService};

#[derive(Clone)]
pub struct MetadataState {
    pub metadata_service: MetadataService,
}

impl MetadataState {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            metadata_service: MetadataService::new(db_conn),
        }
    }
}
