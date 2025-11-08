use axum::Router;
use tokio::net::TcpListener;

pub async fn serve(router: Router) -> anyhow::Result<()> {
    let port = crate::config::get_config().server().port();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on http://localhost:{port}");
    axum::serve(listener, router).await?;

    Ok(())
}
