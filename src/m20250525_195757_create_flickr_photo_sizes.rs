use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250525_191252_create_flickr_photos::FlickrPhoto;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum FlickrPhotoSize {
    Table,
    FlickrPhotoId,
    ServerId,
    Secret,
    Suffix,
    Width,
    Height,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FlickrPhotoSize::Table)
                    .if_not_exists()
                    .col(big_unsigned(FlickrPhotoSize::FlickrPhotoId).not_null())
                    .primary_key(Index::create().col(FlickrPhotoSize::FlickrPhotoId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(FlickrPhotoSize::Table, FlickrPhotoSize::FlickrPhotoId)
                            .to(FlickrPhoto::Table, FlickrPhoto::FlickrId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(FlickrPhotoSize::ServerId).char_len(10))
                    .col(string(FlickrPhotoSize::Secret).char_len(25))
                    .col(string(FlickrPhotoSize::Suffix).string_len(10))
                    .col(unsigned(FlickrPhotoSize::Width))
                    .col(unsigned(FlickrPhotoSize::Height))
                    .col(date_time(FlickrPhotoSize::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(FlickrPhotoSize::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FlickrPhotoSize::Table).to_owned())
            .await
    }
}
