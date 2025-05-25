use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
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

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::photo_tags::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::photo_tags::Relation::Photo.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
