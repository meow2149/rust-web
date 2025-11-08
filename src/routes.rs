use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::{api::user, middleware, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new().nest("/users", user::router::router());

    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .nest("/api", api_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::cors::layer())
}
