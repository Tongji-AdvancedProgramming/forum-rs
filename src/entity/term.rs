use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 学期信息表
#[derive(Debug, Clone, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "term")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// "2022/2023/2"形式的学期表示
    #[sea_orm(primary_key)]
    pub term_no: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
