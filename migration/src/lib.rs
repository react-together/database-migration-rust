pub use sea_orm_migration::prelude::*;

mod m20250514_135430_create_users;
mod m20250515_160120_create_photos;
mod m20250515_160714_create_photo_reactions;
mod m20250517_080512_create_tags;
mod m20250517_080513_create_photo_tags;
mod m20250525_190914_create_directories;
mod m20250525_191252_create_flickr_photos;
mod m20250525_195757_create_flickr_photo_sizes;
mod m20250525_205645_create_flickr_photosets;
mod m20250525_210220_create_flickr_photoset_tags;
mod m20250525_210752_create_photo_files;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250514_135430_create_users::Migration),
            Box::new(m20250515_160120_create_photos::Migration),
            Box::new(m20250515_160714_create_photo_reactions::Migration),
            Box::new(m20250517_080512_create_tags::Migration),
            Box::new(m20250517_080513_create_photo_tags::Migration),
            Box::new(m20250525_190914_create_directories::Migration),
            Box::new(m20250525_191252_create_flickr_photos::Migration),
            Box::new(m20250525_195757_create_flickr_photo_sizes::Migration),
            Box::new(m20250525_205645_create_flickr_photosets::Migration),
            Box::new(m20250525_210220_create_flickr_photoset_tags::Migration),
            Box::new(m20250525_210752_create_photo_files::Migration),
        ]
    }
}

#[async_std::main]
pub async fn main() {
    cli::run_cli(Migrator).await;
}
