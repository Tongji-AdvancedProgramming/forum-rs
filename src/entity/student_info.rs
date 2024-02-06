use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 论坛运行所需要的其他数据
#[derive(Debug, Clone, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "student_info")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// 学生学号
    #[sea_orm(primary_key)]
    pub stu_no: String,

    /// 签名档
    pub description: String,

    /// 昵称（如有，和实名同时显示）
    pub nickname: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
