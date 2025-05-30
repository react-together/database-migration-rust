use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "directories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
        belongs_to = "super::directories::Entity",
        from = "Column::ParentId",
        to = "super::directories::Column::Id"
    )]
    Parent,
}

impl Related<super::directories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Parent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
