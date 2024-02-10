pub mod get {
    use axum::extract::State;
    use forum_macros::forum_handler;

    use crate::{
        entity::tag, service::metadata_service::MetadataServiceTrait,
        state::metadata_state::MetadataState,
    };

    #[forum_handler]
    pub async fn tags(State(state): State<MetadataState>) -> Vec<tag::Model> {
        state.metadata_service.get_tags().await
    }
}
