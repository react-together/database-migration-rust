use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250515_160120_create_photos::Photo, m20250517_080512_create_tags::Tag};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum PhotoTag {
    Table,
    PhotoId,
    TagId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhotoTag::Table)
                    .if_not_exists()
                    .col(big_unsigned(PhotoTag::PhotoId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhotoTag::Table, PhotoTag::PhotoId)
                            .to(Photo::Table, Photo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(PhotoTag::TagId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhotoTag::Table, PhotoTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(Index::create().col(PhotoTag::PhotoId).col(PhotoTag::TagId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhotoTag::Table).to_owned())
            .await
    }
}
