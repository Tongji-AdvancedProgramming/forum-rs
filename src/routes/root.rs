use std::sync::Arc;

use axum::routing::{get, IntoMakeService};
use axum::Router;
use axum_login::{login_required, AuthManagerLayer};
use tower_http::trace::TraceLayer;
use tower_sessions::SessionStore;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::database::Db;

use crate::handler::swagger_handler::ApiDoc;
use crate::routes::{auth, user};
use crate::service::auth::AuthBackend;
use crate::state::user_state::UserState;

pub fn routes(
    db_conn: Arc<Db>,
    auth_layer: AuthManagerLayer<AuthBackend, impl SessionStore + Clone>,
) -> IntoMakeService<Router> {
    let merged_router = {
        let user_state = UserState::new(&db_conn);

        Router::new()
            .nest("/user", user::routes().with_state(user_state))
            .route_layer(login_required!(AuthBackend))
            .merge(auth::routes())
            .route(
                "/",
                get(|| async {
                    "本Rest API应用不提供前端接口。请检查访问地址是否正确。"
                }),
            )
            .merge(Router::new().route("/health", get(|| async { "Healthy" })))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    };

    let app_router = Router::new()
        .merge(merged_router)
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http());

    app_router.into_make_service()
}
