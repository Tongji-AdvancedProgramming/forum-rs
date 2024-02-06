use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 标签名称索引表
#[derive(Debug, Clone, Default, Deserialize, Serialize, DeriveEntityModel, utoipa::ToSchema)]
#[sea_orm(table_name = "tag")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    /// post表中tag的字段名
    #[sea_orm(primary_key)]
    pub tag_field_name: String,

    /// tag的中文解释
    pub tag_name: String,

    /// 对应tar的前景色(FF0000 - RGB方式表示的颜色,每两位表示一个16进制的颜色)
    #[sea_orm(column_name = "tag_fgcolor")]
    #[serde(rename = "tagFgcolor")]
    pub tag_fg_color: String,

    /// 对应tar的背景色(00FF00 - RGB方式表示的颜色,每两位表示一个16进制的颜色)
    #[sea_orm(column_name = "tag_bgcolor")]
    #[serde(rename = "tagBgcolor")]
    pub tag_bg_color: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
