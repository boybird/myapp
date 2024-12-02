#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241202_173429_posts;
mod m20240104_000001_alter_posts_user_id_to_uuid;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241202_173429_posts::Migration),
            Box::new(m20240104_000001_alter_posts_user_id_to_uuid::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}