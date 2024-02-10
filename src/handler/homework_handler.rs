pub mod get {
    use axum::extract::{Query, State};
    use forum_macros::forum_handler;
    use serde::Deserialize;
    use utoipa::ToSchema;

    use crate::{
        entity::homework, service::homework_service::HomeworkServiceTrait,
        state::homework_state::HomeworkState,
    };

    #[derive(Debug, Clone, Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct HomeworkParam {
        pub term: String,
        pub course_no: String,
        pub hw_id: i32,
    }

    #[forum_handler]
    pub async fn homework(
        State(state): State<HomeworkState>,
        Query(param): Query<HomeworkParam>,
    ) -> Option<homework::Model> {
        state
            .homework_service
            .get_homework(&param.term, param.hw_id, &param.course_no)
            .await
    }
}
