use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub stu_term: String,

    pub stu_grade: String,

    pub stu_no: String,

    pub stu_name: String,

    pub stu_sex: String,

    pub stu_password: String,

    #[sqlx(rename = "stu_class_fname")]
    #[serde(rename = "stuClassFname")]
    pub stu_class_full_name: String,

    #[sqlx(rename = "stu_class_sname")]
    #[serde(rename = "stuClassSname")]
    pub stu_class_short_name: String,

    #[sqlx(rename = "stu_userlevel")]
    #[serde(rename = "stuUserlevel")]
    pub stu_user_level: String,

    pub stu_enable: String,

    pub stu_add_date: DateTime<Utc>,

    pub stu_cno_1: Option<String>,

    pub stu_cno_1_is_del: String,

    pub stu_cno_2: Option<String>,

    pub stu_cno_2_is_del: String,

    pub stu_cno_3: Option<String>,

    pub stu_cno_3_is_del: String,

    pub stu_is_del: String,

    pub stu_comment: Option<String>,
}
