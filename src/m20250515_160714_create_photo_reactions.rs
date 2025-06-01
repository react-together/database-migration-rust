use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20250514_135430_create_users::User, m20250515_160120_create_photos::Photo};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum PhotoReaction {
    Table,
    PhotoId,
    UserId,
    IsRecommended,
    Comment,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhotoReaction::Table)
                    .if_not_exists()
                    .col(big_unsigned(PhotoReaction::PhotoId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhotoReaction::Table, PhotoReaction::PhotoId)
                            .to(Photo::Table, Photo::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(big_unsigned(PhotoReaction::UserId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PhotoReaction::Table, PhotoReaction::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(PhotoReaction::PhotoId)
                            .col(PhotoReaction::UserId),
                    )
                    .col(boolean(PhotoReaction::IsRecommended).not_null())
                    .col(text(PhotoReaction::Comment).null())
                    .col(date_time(PhotoReaction::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(PhotoReaction::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhotoReaction::Table).to_owned())
            .await
    }
}
