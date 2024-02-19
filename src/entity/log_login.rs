use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 用户登录日志表
#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "log_login")]
#[serde(default, rename_all = "camelCase")]
pub struct Model {
    /// 序号(主键,自动增长)
    #[sea_orm(primary_key)]
    pub log_login_id: i32,

    /// 学号
    pub log_login_no: String,

    /// 登录IP
    pub log_login_ipaddr: String,

    /// 登录时间
    pub log_login_date: NaiveDateTime,

    /// 登录环境(浏览器的agent)
    pub log_login_useragent: String,

    /// 备注
    pub log_login_comment: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
