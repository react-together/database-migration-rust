use entity::{flickr_photoset_tags::*, flickr_photosets, tags};
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
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entity, Column::FlickrId)
                            .to(flickr_photosets::Entity, flickr_photosets::Column::FlickrId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(Column::TagId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entity, Column::TagId)
                            .to(tags::Entity, tags::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(Index::create().col(Column::FlickrId).col(Column::TagId))
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
