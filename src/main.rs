mod db;
mod entity;
mod error;
mod middleware;
mod migration;
mod routes;
mod state;

use std::{net::SocketAddr, str::FromStr};

use axum::{Router, http::Request, routing::get};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, info};

use crate::{routes::user::router as user_router, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracing();

    let db = db::connect().await?;
    migration::run(&db).await?;
    let state = AppState::new(db);

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

    let app = Router::new()
        .nest("/users", user_router())
        .route("/healthz", get(|| async { "ok" }))
        .layer(
            ServiceBuilder::new()
                .layer(middleware::request_id::layer())
                .layer(trace_layer),
        )
        .with_state(state);

    let addr = std::env::var("APP_ADDR").expect("APP_ADDR environment variable must be set");
    let addr = SocketAddr::from_str(&addr)?;

    info!("listening on {addr}");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    let env_filter =
        EnvFilter::try_from_default_env().expect("RUST_LOG environment variable must be set");

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_target(true).with_level(true))
        .init();
}
