use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Tag {
    Table,
    Id,
    Name,
    Description,
    Note,
    TagType,
}

#[derive(Iden, EnumIter)]
pub enum TagType {
    #[iden = "category"]
    Category,
    #[iden = "photographer"]
    Photographer,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(pk_auto(Tag::Id).big_unsigned())
                    .col(string(Tag::Name).not_null().unique_key().string_len(15))
                    .col(string(Tag::Description).string_len(256))
                    .col(string(Tag::Note).string_len(256))
                    .col(enumeration(
                        Tag::TagType,
                        Alias::new("tag_type"),
                        TagType::iter(),
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await
    }
}
