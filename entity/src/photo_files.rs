use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "photo_files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub photo_id: u64,
    pub directory_id: u64,
    pub name: String,
    pub integrity: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::photos::Entity",
        from = "Column::PhotoId",
        to = "super::photos::Column::Id"
    )]
    Photo,
    #[sea_orm(
        belongs_to = "super::directories::Entity",
        from = "Column::DirectoryId",
        to = "super::directories::Column::Id"
    )]
    Directory,
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photo.def()
    }
}

impl Related<super::directories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Directory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
