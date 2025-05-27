use entity::{flickr_photosets::*, users};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(big_unsigned(Column::FlickrId).not_null())
                    .primary_key(Index::create().col(Column::FlickrId))
                    .col(big_unsigned(Column::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entity, Column::UserId)
                            .to(users::Entity, users::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Column::Title).string_len(256))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity.into_table_ref()).to_owned())
            .await
    }
}
