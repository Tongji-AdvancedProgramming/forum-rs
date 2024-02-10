pub mod get {
    use axum::extract::State;
    use forum_macros::forum_handler;

    use crate::{
        entity::tag, service::metadata_service::MetadataServiceTrait,
        state::metadata_state::MetadataState,
    };

    /// 获取标签列表
    #[utoipa::path(
        get,
        path = "/meta/tags",
        tag = "Metadata",
        responses(
            (status = 200, description = "获取标签列表成功", body = inline(Vec<tag::Model>))
        ),
    )]
    #[forum_handler]
    pub async fn tags(State(state): State<MetadataState>) -> Vec<tag::Model> {
        state.metadata_service.get_tags().await
    }
}
