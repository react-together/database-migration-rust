use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, DeriveActiveEnum, EnumIter, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tag_type")]
pub enum TagType {
    #[sea_orm(string_value = "Category")]
    Category,
    #[sea_orm(string_value = "Photographer")]
    Photographer,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub note: String,
    pub tag_type: TagType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
