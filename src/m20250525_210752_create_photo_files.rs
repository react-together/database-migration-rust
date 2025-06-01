use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250515_160120_create_photos::Photo, m20250525_190914_create_directories::Directory,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum PhotoFile {
    Table,
    PhotoId,
    DirectoryId,
    Name,
    Integrity,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhotoFile::Table)
                    .if_not_exists()
                    .col(big_unsigned(PhotoFile::PhotoId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhotoFile::Table, PhotoFile::PhotoId)
                            .to(Photo::Table, Photo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(PhotoFile::DirectoryId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhotoFile::Table, PhotoFile::DirectoryId)
                            .to(Directory::Table, Directory::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(PhotoFile::Name).string_len(256))
                    .col(string(PhotoFile::Integrity).char_len(64))
                    .col(date_time(PhotoFile::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(PhotoFile::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .primary_key(
                        Index::create()
                            .col(PhotoFile::DirectoryId)
                            .col(PhotoFile::Integrity),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhotoFile::Table).to_owned())
            .await
    }
}
