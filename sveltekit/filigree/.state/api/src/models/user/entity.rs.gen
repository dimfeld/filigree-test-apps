#![allow(unused_imports, dead_code)]
use filigree::auth::{AuthInfo as _, ObjectPermission};
use sea_orm::{prelude::*, DeleteMany, DeleteOne, Insert, Select, UpdateMany, UpdateOne};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{UserId, CREATE_PERMISSION, OWNER_PERMISSION, READ_PERMISSION, WRITE_PERMISSION};
use crate::{auth::AuthInfo, error::Error, models::organization::OrganizationId};

#[derive(
    Serialize, Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, DeriveEntityModel,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: UserId,
    pub organization_id: Option<crate::models::organization::OrganizationId>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

impl sea_orm::ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, sea_orm::DeriveRelation)]
pub enum Relation {}

/// An Entity class that automatically checks for authorization on any query. This structure
/// follows the function names defined by the `sea_orm::EntityTrait` trait, which allows similar
/// usage patterns, except that the methods have a `&self` argument which lets them reference the
/// [AuthInfo].
///
/// The original `Entity` methods are also present with `_unauthed` appended to their names, for
/// the occasional cases where you are sure it's ok to skip the auth check.
///
pub struct AuthedEntity<'a> {
    auth_info: &'a AuthInfo,
}

impl<'a> AuthedEntity<'a> {
    /// Creates a new `AuthedEntity` instance with the given `AuthInfo`.
    pub fn new(auth_info: &'a AuthInfo) -> Self {
        Self { auth_info }
    }

    /// Defines a "belongs to" relationship between the User and the related entity.
    pub fn belongs_to<R>(related: R) -> sea_orm::RelationBuilder<Entity, R>
    where
        R: EntityTrait,
    {
        Entity::belongs_to(related)
    }

    /// Defines a "has one" relationship between the User and the related entity.
    pub fn has_one<R>(related: R) -> sea_orm::RelationBuilder<Entity, R>
    where
        R: EntityTrait + Related<Entity>,
    {
        Entity::has_one(related)
    }

    /// Defines a "has many" relationship between the User and the related entity.
    pub fn has_many<R>(related: R) -> sea_orm::RelationBuilder<Entity, R>
    where
        R: EntityTrait + Related<Entity>,
    {
        Entity::has_many(related)
    }

    /// Performs an unauthenticated find operation on Users.
    pub fn find_unauthed() -> Select<Entity> {
        Entity::find()
    }

    /// Performs an unauthenticated find operation by ID on User.
    pub fn find_by_id_unauthed<T>(values: T) -> Select<Entity>
    where
        T: Into<<<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        Entity::find_by_id(values)
    }

    /// Performs an unauthenticated insert operation for a single User.
    pub fn insert_unauthed<A>(model: A) -> sea_orm::Insert<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        Entity::insert(model)
    }

    /// Performs an unauthenticated insert operation for multiple Users.
    pub fn insert_many_unauthed<A, I>(models: I) -> sea_orm::Insert<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
        I: IntoIterator<Item = A>,
    {
        Entity::insert_many(models)
    }

    /// Performs an unauthenticated update operation for a single User.
    pub fn update_unauthed<A>(model: A) -> sea_orm::UpdateOne<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        Entity::update(model)
    }

    /// Performs an unauthenticated update operation for multiple Users.
    pub fn update_many_unauthed() -> sea_orm::UpdateMany<Entity> {
        Entity::update_many()
    }

    /// Performs an unauthenticated delete operation for a single User.
    pub fn delete_unauthed<A>(model: A) -> sea_orm::DeleteOne<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        Entity::delete(model)
    }

    /// Performs an unauthenticated delete operation for multiple Users.
    pub fn delete_many_unauthed() -> sea_orm::DeleteMany<Entity> {
        Entity::delete_many()
    }

    /// Performs an unauthenticated delete operation by ID.
    pub fn delete_by_id_unauthed<T>(values: T) -> sea_orm::DeleteMany<Entity>
    where
        T: Into<<<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        Entity::delete_by_id(values)
    }

    /// Return the amount of access that the user has to this entity.
    pub fn get_access(&self, id: &UserId) -> ObjectPermission {
        if self.auth_info.has_permission(OWNER_PERMISSION) {
            ObjectPermission::Owner
        } else if self.auth_info.has_permission(WRITE_PERMISSION) {
            ObjectPermission::Write
        } else if self.auth_info.has_permission(READ_PERMISSION) {
            ObjectPermission::Read
        } else {
            ObjectPermission::None
        }
    }
}
