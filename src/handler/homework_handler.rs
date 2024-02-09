pub mod get {
    use axum::extract::{Query, State};
    use serde::Deserialize;
    use utoipa::ToSchema;

    use crate::{
        entity::homework, error::api_error::ApiError, response::api_response::ApiResponse,
        service::homework_service::HomeworkServiceTrait, state::homework_state::HomeworkState,
    };

    #[derive(Debug, Clone, Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct HomeworkParam {
        pub term: String,
        pub course_no: String,
        pub hw_id: i32,
    }

    pub async fn homework(
        State(state): State<HomeworkState>,
        Query(param): Query<HomeworkParam>,
    ) -> Result<ApiResponse<Option<homework::Model>>, ApiError> {
        state
            .homework_service
            .get_homework(&param.term, param.hw_id, &param.course_no)
            .await
            .map(|v| ApiResponse::ok(v))
            .map_err(|err| err.into())
    }
}
