use entity::sea_orm_active_enums::TagType;
use entity::tags::*;
use sea_orm_migration::{prelude::*, schema::*, sea_orm::Iterable};

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
                    .col(pk_auto(Column::Id).big_unsigned())
                    .col(string(Column::Name).not_null().unique_key().string_len(15))
                    .col(string(Column::Description).string_len(256))
                    .col(string(Column::Note).string_len(256))
                    .col(enumeration(
                        Column::TagType,
                        Alias::new("tag_type"),
                        TagType::iter(),
                    ))
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
