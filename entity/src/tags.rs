use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Serialize, Deserialize, DeriveActiveEnum, DeriveIden, EnumIter, PartialEq, Eq,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tag_type")]
pub enum TagType {
    #[sea_orm(string_value = "Category")]
    Category,
    #[sea_orm(string_value = "Photographer")]
    Photographer,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub note: String,
    pub tag_type: TagType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::photo_tags::Entity")]
    PhotoTags,
}

impl Related<super::photo_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PhotoTags.def()
    }
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        super::photo_tags::Relation::Photo.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::photo_tags::Relation::Tag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
