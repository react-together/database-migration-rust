use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(unique, column_type = "String(StringLen::N(50))")]
    pub name: String,
    #[sea_orm(unique, column_type = "String(StringLen::N(255))")]
    pub account: String,
    #[sea_orm(unique, column_type = "String(StringLen::N(320))")]
    pub email: String,
    #[sea_orm(unique, column_type = "String(StringLen::N(36))")]
    pub keycloak_sub: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
