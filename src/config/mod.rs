mod database;
mod server;

use std::sync::LazyLock;

use anyhow::Context;
use config::{Config, FileFormat};
use serde::Deserialize;

pub use database::DatabaseConfig;
pub use server::ServerConfig;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load configuration"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("config.toml")
                    .format(FileFormat::Toml)
                    .required(true),
            )
            .add_source(config::Environment::with_prefix("APP").separator("_"))
            .build()
            .with_context(|| anyhow::anyhow!("Failed to build configuration"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize configuration"))
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

pub fn get_config() -> &'static AppConfig {
    &CONFIG
}
