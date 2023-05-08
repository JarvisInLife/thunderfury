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
                    .col(ColumnDef::new(Tv::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tv::Title).string().not_null())
                    .col(ColumnDef::new(Tv::Year).integer().not_null())
                    .col(ColumnDef::new(Tv::Status).string().not_null())
                    .col(ColumnDef::new(Tv::OriginalLanguage).string().not_null())
                    .col(ColumnDef::new(Tv::OriginalTitle).string().not_null())
                    .col(ColumnDef::new(Tv::Overview).string().not_null())
                    .col(ColumnDef::new(Tv::NumberOfSeasons).integer().not_null())
                    .col(ColumnDef::new(Tv::NumberOfEpisodes).integer().not_null())
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
    Title,
    Year,
    Status,
    OriginalLanguage,
    OriginalTitle,
    Overview,
    NumberOfSeasons,
    NumberOfEpisodes,
}
