use sea_orm::entity::prelude::*;
use serde::Serialize;
// 生成实体
use chrono::DateTime;
use chrono::Utc;
use sea_orm::EntityTrait;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::Post::Entity")]
    Post,
}

impl Related<super::Post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        todo!()
    }

    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait
    {
        todo!()
    }

    async fn after_save<C>(model: <Self::Entity as EntityTrait>::Model, db: &C, insert: bool) -> Result<<Self::Entity as EntityTrait>::Model, DbErr>
    where
        C: ConnectionTrait
    {
        todo!()
    }

    async fn before_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait
    {
        todo!()
    }

    async fn after_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait
    {
        todo!()
    }
}