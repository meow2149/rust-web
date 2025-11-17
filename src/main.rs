mod config;
mod database;
mod dto;
mod entity;
mod error;
mod handler;
mod logger;
mod middleware;
mod migration;
mod routes;
mod serve;
mod state;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let db = database::init().await?;
    migration::run(&db).await?;

    let state = AppState::new(db);
    let app = routes::create_router(state);

    serve::serve(app).await?;

    Ok(())
}
