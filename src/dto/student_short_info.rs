use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default, ToSchema, FromQueryResult)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct StudentShortInfo {
    pub nick_name: String,
    pub real_name: String,
    pub description: String,
    pub stu_no: String,
    pub major: String,
    pub role: String,
}
