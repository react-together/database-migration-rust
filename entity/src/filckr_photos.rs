use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flickr_photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub flickr_id: u64,
    pub photo_id: u64,
    pub is_public: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
        belongs_to = "super::photos::Entity",
        from = "Column::PhotoId",
        to = "super::photos::Column::Id"
    )]
    Photo,
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
