pub use sea_orm_migration::prelude::*;

mod m20250514_135430_create_users;
mod m20250515_160120_create_photos;
mod m20250515_160714_create_photo_reactions;
mod m20250517_080512_create_tags;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250514_135430_create_users::Migration),
            Box::new(m20250515_160120_create_photos::Migration),
            Box::new(m20250515_160714_create_photo_reactions::Migration),
            Box::new(m20250517_080512_create_tags::Migration),
        ]
    }
}

#[async_std::main]
pub async fn main() {
    cli::run_cli(Migrator).await;
}
