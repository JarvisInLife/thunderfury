use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;

mod m20230428_165822_create_tv_table;
mod m20230510_050508_create_season_table;
mod m20230510_051935_create_episode_table;
mod m20230602_104150_create_movie_table;
mod m20230602_104200_create_genre_table;
mod m20230602_104209_create_subscription_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230428_165822_create_tv_table::Migration),
            Box::new(m20230510_050508_create_season_table::Migration),
            Box::new(m20230510_051935_create_episode_table::Migration),
            Box::new(m20230602_104150_create_movie_table::Migration),
            Box::new(m20230602_104200_create_genre_table::Migration),
            Box::new(m20230602_104209_create_subscription_table::Migration),
        ]
    }
}

pub async fn fresh(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::fresh(db).await
}

pub async fn up(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}
