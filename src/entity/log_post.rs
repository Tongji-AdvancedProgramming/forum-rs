use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 发帖日志表
#[derive(Debug, Clone, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "log_post")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// 序号(主键,自动增长)
    #[sea_orm(primary_key)]
    pub log_post_id: i32,

    /// 帖子id
    #[sea_orm(column_name = "log_post_postid")]
    #[serde(rename = "logPostPostid")]
    pub log_post_post_id: i32,

    /// 操作人学号
    #[sea_orm(column_name = "log_post_opno")]
    #[serde(rename = "logPostOpno")]
    pub log_post_op_no: String,

    /// 登录IP
    pub log_post_ipaddr: String,

    /// 登录时间
    pub log_post_date: NaiveDateTime,

    /// 备注（没考虑好怎么更合理）
    pub log_post_comment: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
