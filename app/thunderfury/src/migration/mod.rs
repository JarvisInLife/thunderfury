use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

mod m20230428_165822_create_media_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230428_165822_create_media_table::Migration)]
    }
}

pub async fn fresh(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::fresh(db).await
}

pub async fn up(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}
