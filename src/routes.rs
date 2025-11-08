use axum::{Router, http::Request, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{api::user, middleware, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let request_id = request
                .headers()
                .get(middleware::request_id::REQUEST_ID_HEADER)
                .and_then(|value| value.to_str().ok())
                .unwrap_or("unknown");

            tracing::span!(
                Level::INFO,
                "http_request",
                method = %request.method(),
                uri = %request.uri(),
                request_id,
            )
        })
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .include_headers(true),
        );

    let api_routes = Router::new().nest("/users", user::router::router());

    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .nest("/api", api_routes)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(middleware::request_id::layer())
                .layer(trace_layer),
        )
        .layer(middleware::cors::layer())
}
