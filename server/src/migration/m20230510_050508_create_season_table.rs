use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Season::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Season::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Season::TvId).integer().not_null())
                    .col(ColumnDef::new(Season::SeasonNumber).integer().not_null())
                    .col(ColumnDef::new(Season::Year).integer().not_null())
                    .col(ColumnDef::new(Season::Status).string().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_tv_season")
                            .col(Season::TvId)
                            .col(Season::SeasonNumber),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Season::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Season {
    Table,
    Id,
    TvId,
    SeasonNumber,
    Year,
    Status,
}
