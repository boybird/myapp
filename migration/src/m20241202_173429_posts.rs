use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Posts::Table)
                    .col(pk_auto(Posts::Id))
                    .col(string_null(Posts::Title))
                    .col(text_null(Posts::Content))
                    .col(string_null(Posts::Summary))
                    .col(boolean_null(Posts::Published))
                    .col(string_null(Posts::Slug))
                    .col(integer_null(Posts::UserId))
                    .col(timestamp_with_time_zone_null(Posts::PublishedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    Content,
    Summary,
    Published,
    Slug,
    UserId,
    PublishedAt,
    
}

