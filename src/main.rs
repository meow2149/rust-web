mod api;
mod config;
mod database;
mod error;
mod logger;
mod middleware;
mod migration;
mod routes;
mod serve;
mod state;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    logger::init();

    let db = database::init().await?;
    migration::run(&db).await?;

    let state = AppState::new(db);
    let app = routes::create_router(state);

    serve::serve(app).await?;

    Ok(())
}
