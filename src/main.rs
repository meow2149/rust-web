mod api;
mod app;
mod db;
mod error;
mod logging;
mod middleware;
mod migration;
mod server;
mod state;

use dotenvy::dotenv;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logging::init();

    let db = db::connect().await?;
    migration::run(&db).await?;

    let state = AppState::new(db);
    let app = app::create_router(state);

    server::serve(app).await?;

    Ok(())
}
