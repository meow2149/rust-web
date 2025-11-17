use axum::{Router, routing};
use tower_http::trace::TraceLayer;

use crate::{handler, middleware, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/healthz", routing::get(|| async { "ok" }))
                .nest(
                    "/users",
                    Router::new()
                        .route(
                            "/",
                            routing::get(handler::user::list_users)
                                .post(handler::user::create_user),
                        )
                        .route(
                            "/{id}",
                            routing::get(handler::user::get_user)
                                .put(handler::user::update_user)
                                .delete(handler::user::delete_user),
                        ),
                ),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::cors::layer())
}
