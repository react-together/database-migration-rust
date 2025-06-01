use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250515_160120_create_photos::Photo;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum FlickrPhoto {
    Table,
    FlickrId,
    PhotoId,
    IsPublic,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FlickrPhoto::Table)
                    .if_not_exists()
                    .col(big_unsigned(FlickrPhoto::FlickrId).not_null())
                    .primary_key(Index::create().col(FlickrPhoto::FlickrId))
                    .col(big_unsigned(FlickrPhoto::PhotoId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FlickrPhoto::Table, FlickrPhoto::PhotoId)
                            .to(Photo::Table, Photo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(boolean(FlickrPhoto::IsPublic))
                    .col(date_time(FlickrPhoto::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(FlickrPhoto::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FlickrPhoto::Table).to_owned())
            .await
    }
}
