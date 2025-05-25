use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flickr_photoset_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub flickr_id: u64,
    #[sea_orm(primary_key)]
    pub tag_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::flickr_photosets::Entity",
        from = "Column::FlickrId",
        to = "super::flickr_photosets::Column::FlickrId"
    )]
    FlickrPhotoset,
    #[sea_orm(
        belongs_to = "super::tags::Entity",
        from = "Column::TagId",
        to = "super::tags::Column::Id"
    )]
    Tag,
}

impl Related<super::flickr_photosets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlickrPhotoset.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
