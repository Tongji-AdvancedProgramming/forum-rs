use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 发帖日志表
#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "notification")]
#[serde(default, rename_all(serialize = "camelCase"))]
pub struct Model {
    /// 序号(主键,自动增长)
    #[sea_orm(primary_key)]
    pub ntf_id: i32,

    pub ntf_type: String,

    pub ntf_title: String,

    pub ntf_content: String,

    pub ntf_receiver: String,

    pub ntf_datetime: NaiveDateTime,

    pub ntf_read: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
