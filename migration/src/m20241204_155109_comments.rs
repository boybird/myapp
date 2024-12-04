use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Comments::Table)
                    .col(pk_auto(Comments::Id))
                    .col(text_null(Comments::Content))
                    .col(integer_null(Comments::PostId))
                    .col(uuid_uniq(Comments::UserId))
                    .col(integer_null(Comments::ParentId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    Content,
    PostId,
    UserId,
    ParentId,
    
}

