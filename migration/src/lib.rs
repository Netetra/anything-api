pub use sea_orm_migration::prelude::*;

mod m20240820_013417_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240820_013417_create_user_table::Migration)]
    }
}
