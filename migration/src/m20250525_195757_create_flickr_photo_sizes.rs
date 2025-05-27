use entity::{flickr_photo_sizes::*, flickr_photos};
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
                    .col(big_unsigned(Column::FlickrPhotoId).not_null())
                    .primary_key(Index::create().col(Column::FlickrPhotoId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entity, Column::FlickrPhotoId)
                            .to(flickr_photos::Entity, flickr_photos::Column::FlickrId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Column::ServerId).char_len(10))
                    .col(string(Column::Secret).char_len(25))
                    .col(string(Column::Suffix).string_len(10))
                    .col(unsigned(Column::Width))
                    .col(unsigned(Column::Height))
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
