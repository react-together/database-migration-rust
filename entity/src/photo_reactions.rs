use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "photo_reactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub photo_id: u64,
    #[sea_orm(primary_key)]
    pub user_id: u64,
    pub is_recommended: bool,
    pub comment: Option<String>,
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
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photo.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
