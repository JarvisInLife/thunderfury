use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Movie::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Movie::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Movie::Name).string().not_null())
                    .col(ColumnDef::new(Movie::Year).integer().not_null())
                    .col(ColumnDef::new(Movie::Status).string().not_null())
                    .col(ColumnDef::new(Movie::ReleaseData).string().not_null())
                    .col(ColumnDef::new(Movie::TmdbId).integer().not_null())
                    .col(ColumnDef::new(Movie::Overview).string().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_tmdb_id")
                            .col(Movie::TmdbId),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_name_year")
                            .col(Movie::Name)
                            .col(Movie::Year),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Movie::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Movie {
    Table,
    Id,
    Name,
    Year,
    Status,
    ReleaseData,
    TmdbId,
    Overview,
}
