use std::sync::Arc;

use crate::{config::database::Db, service::board_service::BoardService};

#[derive(Clone)]
pub struct BoardState {
    pub board_service: BoardService,
}

impl BoardState {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            board_service: BoardService::new(db_conn),
        }
    }
}
