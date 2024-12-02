use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer_null, uuid_null};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, drop the existing user_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .drop_column(Posts::UserId)
                    .to_owned(),
            )
            .await?;

        // Then, add the new UUID column
        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .add_column(uuid_null(Posts::UserId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // First, drop the UUID column
        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .drop_column(Posts::UserId)
                    .to_owned(),
            )
            .await?;

        // Then, add back the integer column
        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .add_column(integer_null(Posts::UserId))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    UserId,
}
