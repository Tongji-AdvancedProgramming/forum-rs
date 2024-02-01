use std::sync::Arc;

use axum::routing::{get, IntoMakeService};
use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::database::Db;
use crate::config::session::RedisSession;
use crate::handler::swagger::ApiDoc;
use crate::routes::user;
use crate::state::user_state::UserState;

pub fn routes(db_conn: Arc<Db>, redis_session: RedisSession) -> IntoMakeService<Router> {
    let merged_router = {
        let user_state = UserState::new(&db_conn);

        Router::new()
            .route(
                "/",
                get(|| async {
                    "本Rest API应用不提供前端接口。请检查访问地址是否正确。"
                }),
            )
            .merge(Router::new().route("/health", get(|| async { "Healthy" })))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .nest("/user", user::routes().with_state(user_state))
    };

    let app_router = Router::new()
        .merge(merged_router)
        .layer(redis_session.session_layer)
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}
