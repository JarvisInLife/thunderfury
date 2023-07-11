use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tv::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tv::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tv::Name).string().not_null())
                    .col(ColumnDef::new(Tv::Year).integer().not_null())
                    .col(ColumnDef::new(Tv::Status).string().not_null())
                    .col(ColumnDef::new(Tv::FirstAirDate).string().not_null())
                    .col(ColumnDef::new(Tv::NumberOfSeasons).integer().not_null())
                    .col(ColumnDef::new(Tv::TmdbId).integer().not_null())
                    .col(ColumnDef::new(Tv::Overview).string().not_null())
                    .index(Index::create().unique().name("uk_tmdb_id").col(Tv::TmdbId))
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_name_year")
                            .col(Tv::Name)
                            .col(Tv::Year),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tv::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Tv {
    Table,
    Id,
    Name,
    Year,
    Status,
    FirstAirDate,
    NumberOfSeasons,
    TmdbId,
    Overview,
}
