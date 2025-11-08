use std::{net::SocketAddr, str::FromStr};

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

pub async fn serve(app: Router) -> anyhow::Result<()> {
    let addr = std::env::var("APP_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let addr = SocketAddr::from_str(&addr)?;

    info!("listening on {addr}");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
