use std::time::Duration;

use anyhow::{Context, anyhow};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connect() -> anyhow::Result<DatabaseConnection> {
    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL environment variable not set")?;

    if database_url.trim().is_empty() {
        return Err(anyhow!("DATABASE_URL must not be empty"));
    }

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800));

    let conn = Database::connect(opt).await?;

    Ok(conn)
}
