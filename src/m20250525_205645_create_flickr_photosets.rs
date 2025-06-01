use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250514_135430_create_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum FlickrPhotoset {
    Table,
    FlickrId,
    UserId,
    Title,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FlickrPhotoset::Table)
                    .if_not_exists()
                    .col(big_unsigned(FlickrPhotoset::FlickrId).not_null())
                    .primary_key(Index::create().col(FlickrPhotoset::FlickrId))
                    .col(big_unsigned(FlickrPhotoset::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(FlickrPhotoset::Table, FlickrPhotoset::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(FlickrPhotoset::Title).string_len(256))
                    .col(string(FlickrPhotoset::Description).string_len(500))
                    .col(date_time(FlickrPhotoset::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(FlickrPhotoset::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FlickrPhotoset::Table).to_owned())
            .await
    }
}
