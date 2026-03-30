pub use sea_orm_migration::prelude::*;

mod m20260330_183859_create_table_users;
mod m20260330_183909_create_table_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260330_183859_create_table_users::Migration),
            Box::new(m20260330_183909_create_table_posts::Migration),
        ]
    }
}
