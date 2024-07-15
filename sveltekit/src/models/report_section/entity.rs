#![allow(unused_imports, dead_code)]
use filigree::auth::{AuthInfo as _, ObjectPermission};
use sea_orm::{prelude::*, DeleteMany, DeleteOne, Insert, Select, UpdateMany, UpdateOne};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{
    ReportSectionId, CREATE_PERMISSION, OWNER_PERMISSION, READ_PERMISSION, WRITE_PERMISSION,
};
use crate::{
    auth::AuthInfo,
    error::Error,
    models::{organization::OrganizationId, report::ReportId},
};

#[derive(
    Serialize, Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, DeriveEntityModel,
)]
#[sea_orm(table_name = "report_sections")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: ReportSectionId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub viz: String,
    pub options: serde_json::Value,
    pub report_id: ReportId,
}

impl sea_orm::ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, sea_orm::DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::report::entity::Entity",
        from = "Column::ReportId",
        to = "crate::models::report::Column::Id"
    )]
    Report,
}

impl Related<crate::models::report::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Report.def()
    }
}

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

    /// Defines a "belongs to" relationship between the ReportSection and the related entity.
    pub fn belongs_to<R>(related: R) -> sea_orm::RelationBuilder<Entity, R>
    where
        R: EntityTrait,
    {
        Entity::belongs_to(related)
    }

    /// Defines a "has one" relationship between the ReportSection and the related entity.
    pub fn has_one<R>(related: R) -> sea_orm::RelationBuilder<Entity, R>
    where
        R: EntityTrait + Related<Entity>,
    {
        Entity::has_one(related)
    }

    /// Defines a "has many" relationship between the ReportSection and the related entity.
    pub fn has_many<R>(related: R) -> sea_orm::RelationBuilder<Entity, R>
    where
        R: EntityTrait + Related<Entity>,
    {
        Entity::has_many(related)
    }

    /// Performs an authenticated find operation on ReportSection
    pub fn find(&self) -> Result<Select<Entity>, error_stack::Report<Error>> {
        self.check_list(Entity::find())
    }

    /// Performs an authenticated find operation by ID on ReportSection
    pub fn find_by_id<T>(&self, values: T) -> Result<Select<Entity>, error_stack::Report<Error>>
    where
        T: Into<<<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        self.check_get(Entity::find_by_id(values))
    }

    /// Performs an authenticated insert operation for a single ReportSection.
    pub fn insert<A>(&self, model: A) -> Result<sea_orm::Insert<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        self.check_insert(Entity::insert(model))
    }

    /// Performs an authenticated insert operation for multiple ReportSections.
    pub fn insert_many<A, I>(
        &self,
        models: I,
    ) -> Result<sea_orm::Insert<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
        I: IntoIterator<Item = A>,
    {
        self.check_insert(Entity::insert_many(models))
    }

    /// Performs an authenticated update operation for a single ReportSection.
    pub fn update<A>(&self, model: A) -> Result<sea_orm::UpdateOne<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        // TODO clear fields that are owner-only if this the user only had normal access
        self.check_update_one(Entity::update(model))
    }

    /// Performs an authenticated update operation for multiple ReportSections.
    pub fn update_many(&self) -> Result<sea_orm::UpdateMany<Entity>, error_stack::Report<Error>> {
        self.check_update_many(Entity::update_many())
    }

    /// Performs an authenticated delete operation for a single ReportSection.
    pub fn delete<A>(&self, model: A) -> Result<sea_orm::DeleteOne<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        self.check_delete_one(Entity::delete(model))
    }

    /// Performs an authenticated delete operation for multiple ReportSections.
    pub fn delete_many(&self) -> Result<sea_orm::DeleteMany<Entity>, error_stack::Report<Error>> {
        self.check_delete_many(Entity::delete_many())
    }

    /// Performs an authenticated delete operation by ID.
    pub fn delete_by_id<T>(
        &self,
        values: T,
    ) -> Result<sea_orm::DeleteMany<Entity>, error_stack::Report<Error>>
    where
        T: Into<<<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        self.check_delete_many(Entity::delete_by_id(values))
    }

    /// Performs an unauthenticated find operation on ReportSections.
    pub fn find_unauthed() -> Select<Entity> {
        Entity::find()
    }

    /// Performs an unauthenticated find operation by ID on ReportSection.
    pub fn find_by_id_unauthed<T>(values: T) -> Select<Entity>
    where
        T: Into<<<Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        Entity::find_by_id(values)
    }

    /// Performs an unauthenticated insert operation for a single ReportSection.
    pub fn insert_unauthed<A>(model: A) -> sea_orm::Insert<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        Entity::insert(model)
    }

    /// Performs an unauthenticated insert operation for multiple ReportSections.
    pub fn insert_many_unauthed<A, I>(models: I) -> sea_orm::Insert<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
        I: IntoIterator<Item = A>,
    {
        Entity::insert_many(models)
    }

    /// Performs an unauthenticated update operation for a single ReportSection.
    pub fn update_unauthed<A>(model: A) -> sea_orm::UpdateOne<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        Entity::update(model)
    }

    /// Performs an unauthenticated update operation for multiple ReportSections.
    pub fn update_many_unauthed() -> sea_orm::UpdateMany<Entity> {
        Entity::update_many()
    }

    /// Performs an unauthenticated delete operation for a single ReportSection.
    pub fn delete_unauthed<A>(model: A) -> sea_orm::DeleteOne<A>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        Entity::delete(model)
    }

    /// Performs an unauthenticated delete operation for multiple ReportSections.
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
    pub fn get_access(&self, id: &ReportSectionId) -> ObjectPermission {
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

    /// Adds a clause that filters a list-style query based on the user's permissions and organization.
    pub fn check_list(
        &self,
        select: Select<Entity>,
    ) -> Result<Select<Entity>, error_stack::Report<Error>> {
        if self.auth_info.has_permission(READ_PERMISSION) {
            Ok(select.filter(Column::OrganizationId.eq(self.auth_info.organization_id.clone())))
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                READ_PERMISSION,
            )))
        }
    }

    /// Adds a clause that filters a get-style query based on the user's permissions and organization.
    pub fn check_get(
        &self,
        select: Select<Entity>,
    ) -> Result<Select<Entity>, error_stack::Report<Error>> {
        if self.auth_info.has_permission(READ_PERMISSION) {
            Ok(select.filter(Column::OrganizationId.eq(self.auth_info.organization_id.clone())))
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                READ_PERMISSION,
            )))
        }
    }

    /// Adds a clause that filters an update operation for a single entity based on the user's permissions and organization.
    pub fn check_update_one<A>(
        &self,
        update: UpdateOne<A>,
    ) -> Result<UpdateOne<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        if self.auth_info.has_permission(WRITE_PERMISSION) {
            Ok(update.filter(Column::OrganizationId.eq(self.auth_info.organization_id.clone())))
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                WRITE_PERMISSION,
            )))
        }
    }

    /// Adds a clause that filters an update operation for multiple entities based on the user's permissions and organization.
    pub fn check_update_many(
        &self,
        update: UpdateMany<Entity>,
    ) -> Result<UpdateMany<Entity>, error_stack::Report<Error>> {
        if self.auth_info.has_permission(WRITE_PERMISSION) {
            Ok(update.filter(Column::OrganizationId.eq(self.auth_info.organization_id.clone())))
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                WRITE_PERMISSION,
            )))
        }
    }

    /// Checks if the user has permission to insert and returns the insert operation.
    pub fn check_insert<A>(
        &self,
        insert: Insert<A>,
    ) -> Result<Insert<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        if self.auth_info.has_permission(CREATE_PERMISSION) {
            Ok(insert)
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                CREATE_PERMISSION,
            )))
        }
    }

    /// Adds a clause that filters a delete operation for a single entity based on the user's permissions and organization.
    pub fn check_delete_one<A>(
        &self,
        del: DeleteOne<A>,
    ) -> Result<DeleteOne<A>, error_stack::Report<Error>>
    where
        A: ActiveModelTrait<Entity = Entity>,
    {
        if self.auth_info.has_permission(CREATE_PERMISSION) {
            Ok(del.filter(Column::OrganizationId.eq(self.auth_info.organization_id.clone())))
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                CREATE_PERMISSION,
            )))
        }
    }

    /// Adds a clause that filters a delete operation for multiple entities based on the user's permissions and organization.
    pub fn check_delete_many(
        &self,
        del: DeleteMany<Entity>,
    ) -> Result<DeleteMany<Entity>, error_stack::Report<Error>> {
        if self.auth_info.has_permission(CREATE_PERMISSION) {
            Ok(del.filter(Column::OrganizationId.eq(self.auth_info.organization_id.clone())))
        } else {
            Err(error_stack::Report::new(crate::Error::MissingPermission(
                CREATE_PERMISSION,
            )))
        }
    }
}
