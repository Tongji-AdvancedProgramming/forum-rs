use axum::extract::{Query, State};
use forum_macros::forum_handler;
use serde::Deserialize;

use crate::{
    dto::board::Board, service::board_service::BoardServiceTrait, state::board_state::BoardState,
};

#[derive(Deserialize)]
pub struct GetBoardInfoParam {
    pub id: String,
}

/// 根据板块id获取板块信息
#[utoipa::path(
    get,
    path = "/board",
    tag = "Board",
    responses(
        (status = 200, description = "获取板块信息成功", body = inline(Board))
    ),
    params(
        ("id" = String, Query, description = "板块的ID"),
    ),
)]
#[forum_handler]
pub async fn get_board_info(
    State(state): State<BoardState>,
    Query(param): Query<GetBoardInfoParam>,
) -> Board {
    state.board_service.parse_id_and_fetch(&param.id).await
}
