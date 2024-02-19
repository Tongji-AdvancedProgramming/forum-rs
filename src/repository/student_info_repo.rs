use std::sync::Arc;

use sea_orm::{
    ColumnTrait, DbBackend, EntityTrait, FromQueryResult, JsonValue, QueryFilter, QuerySelect,
    Statement,
};

use crate::{
    config::database::{DatabaseTrait, Db},
    dto::student_short_info::StudentShortInfo,
    entity::student,
    error::api_error::ApiError,
};

pub trait StudentInfoRepositoryTrait {
    type Error: Into<ApiError> + Send + Sync;
    async fn get_student_default_info(
        &self,
        stu_no: &str,
    ) -> Result<Option<student::Model>, Self::Error>;

    async fn get_student_short_info(
        &self,
        stu_no: &str,
    ) -> Result<Option<StudentShortInfo>, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct StudentInfoRepository {
    pub db_conn: Arc<Db>,
}

impl StudentInfoRepository {
    pub fn new(db_conn: &Arc<Db>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}

impl StudentInfoRepositoryTrait for StudentInfoRepository {
    type Error = sea_orm::error::DbErr;
    async fn get_student_default_info(
        &self,
        stu_no: &str,
    ) -> Result<Option<student::Model>, Self::Error> {
        student::Entity::find()
            .select_only()
            .column(student::Column::StuName)
            .filter(student::Column::StuNo.eq(stu_no))
            .into_json()
            .one(self.db_conn.get_db())
            .await
            .map(|v| v.map(|v| serde_json::from_value::<student::Model>(v).unwrap()))
    }

    async fn get_student_short_info(
        &self,
        stu_no: &str,
    ) -> Result<Option<StudentShortInfo>, Self::Error> {
        let sql = r#"
        select si.nickname       as nick_name,
               si.description    as description,
               s.stu_name        as real_name,
               s.stu_no          as stu_no,
               s.stu_class_sname as major,
               s.stu_userlevel   as role
        from student s
                 left join student_info si on s.stu_no = si.stu_no
        where s.stu_no = ?
        limit 1
        "#;

        JsonValue::find_by_statement(Statement::from_sql_and_values(
            DbBackend::MySql,
            sql,
            [stu_no.into()],
        ))
        .into_model::<StudentShortInfo>()
        .one(self.db_conn.get_db())
        .await
    }
}
