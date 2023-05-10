use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Episode::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Episode::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Episode::TvId).integer().not_null())
                    .col(ColumnDef::new(Episode::SeasonNumber).integer().not_null())
                    .col(ColumnDef::new(Episode::EpisodeNumber).integer().not_null())
                    .col(ColumnDef::new(Episode::Title).string().not_null())
                    .col(ColumnDef::new(Episode::Status).string().not_null())
                    .col(ColumnDef::new(Episode::Overview).string().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_tv_season_episode")
                            .col(Episode::TvId)
                            .col(Episode::SeasonNumber)
                            .col(Episode::EpisodeNumber),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Episode::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Episode {
    Table,
    Id,
    TvId,
    SeasonNumber,
    EpisodeNumber,
    Title,
    Status,
    Overview,
}
