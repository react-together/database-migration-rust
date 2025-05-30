use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flickr_photo_sizes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub flickr_photo_id: u64,
    pub server_id: String,
    pub secret: String,
    pub suffix: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::flickr_photos::Entity",
        from = "Column::FlickrPhotoId",
        to = "super::flickr_photos::Column::FlickrId"
    )]
    FlickrPhoto,
}

impl Related<super::flickr_photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FlickrPhoto.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
