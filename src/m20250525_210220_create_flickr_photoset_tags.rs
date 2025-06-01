use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250517_080512_create_tags::Tag, m20250525_205645_create_flickr_photosets::FlickrPhotoset,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum FlickrPhotosetTag {
    Table,
    FlickrId,
    TagId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FlickrPhotosetTag::Table)
                    .if_not_exists()
                    .col(big_unsigned(FlickrPhotosetTag::FlickrId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FlickrPhotosetTag::Table, FlickrPhotosetTag::FlickrId)
                            .to(FlickrPhotoset::Table, FlickrPhotoset::FlickrId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(FlickrPhotosetTag::TagId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FlickrPhotosetTag::Table, FlickrPhotosetTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(FlickrPhotosetTag::FlickrId)
                            .col(FlickrPhotosetTag::TagId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FlickrPhotosetTag::Table).to_owned())
            .await
    }
}
