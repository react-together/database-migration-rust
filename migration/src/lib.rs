pub use sea_orm_migration::prelude::*;

mod m20250407_051016_create_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250407_051016_create_users::Migration)]
    }
}

#[async_std::main]
pub async fn main() {
    cli::run_cli(Migrator).await;
}
