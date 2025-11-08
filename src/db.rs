use anyhow::{Context, anyhow};
use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> anyhow::Result<DatabaseConnection> {
    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;

    if database_url.trim().is_empty() {
        return Err(anyhow!("DATABASE_URL must not be empty"));
    }

    let conn = Database::connect(&database_url).await?;

    Ok(conn)
}
