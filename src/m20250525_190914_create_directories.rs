use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Directory {
    Table,
    Id,
    ParentId,
    Name,
    Path,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Directory::Table)
                    .if_not_exists()
                    .col(pk_auto(Directory::Id).big_unsigned())
                    .col(big_unsigned(Directory::ParentId).null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Directory::Table, Directory::ParentId)
                            .to(Directory::Table, Directory::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Directory::Name).string_len(256))
                    .col(string(Directory::Path).string_len(4096))
                    .col(date_time(Directory::CreatedAt).default(Expr::current_timestamp()))
                    .col(
                        date_time(Directory::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("on update CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Directory::Table).to_owned())
            .await
    }
}
