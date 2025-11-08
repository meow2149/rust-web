use axum::{Router, routing};
use tower_http::trace::TraceLayer;

use crate::{api::user, middleware, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthz", routing::get(|| async { "ok" }))
                .nest("/users", user::router::router()),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::cors::layer())
}
