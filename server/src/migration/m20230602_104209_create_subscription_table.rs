use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscription::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subscription::MediaType).string().not_null())
                    .col(ColumnDef::new(Subscription::MediaId).integer().not_null())
                    .col(ColumnDef::new(Subscription::Status).string().not_null())
                    .col(ColumnDef::new(Subscription::RssUrl).string().not_null())
                    .index(
                        Index::create()
                            .unique()
                            .name("uk_media")
                            .col(Subscription::MediaType)
                            .col(Subscription::MediaId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Subscription {
    Table,
    Id,
    MediaType,
    MediaId,
    Status,
    RssUrl,
}
