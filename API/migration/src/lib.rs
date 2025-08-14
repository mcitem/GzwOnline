pub use sea_orm_migration::prelude::*;

pub mod m20220101_000001_create_table;
mod m20240819_045504_insert_category_data;
mod m20240819_074240_insert_collection_data;
mod m20240819_100811_insert_photo_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240819_045504_insert_category_data::Migration),
            Box::new(m20240819_074240_insert_collection_data::Migration),
            Box::new(m20240819_100811_insert_photo_data::Migration),
        ]
    }
}
