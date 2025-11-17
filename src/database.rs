use std::cmp::max;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::config;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = config::get_config().database();
    let mut options = ConnectOptions::new(format!(
        "postgresql://{}:{}@{}:{}/{}",
        database_config.user(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database(),
    ));
    let cpus = u32::try_from(num_cpus::get())?;
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());

    let database_connection = Database::connect(options).await?;

    database_connection.ping().await?;

    tracing::info!("Database connection initialized");

    Ok(database_connection)
}
