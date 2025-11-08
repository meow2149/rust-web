use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

mod create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(create_users_table::Migration)]
    }
}

pub async fn run(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}
