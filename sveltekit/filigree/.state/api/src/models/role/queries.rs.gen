#![allow(unused_imports, unused_variables, dead_code)]
use std::str::FromStr;

use error_stack::ResultExt;
use filigree::{
    auth::ObjectPermission,
    errors::OrderByError,
    sql::{BindingOperator, FilterBuilder, ValuesBuilder},
};
use sea_orm::{
    entity::ActiveValue,
    prelude::*,
    query::*,
    sea_query::{self, expr::Expr},
};
use serde::Deserialize;
use sqlx::{
    postgres::PgRow, query_file, query_file_as, query_file_scalar, PgConnection, PgExecutor,
};
use tracing::{event, instrument, Level};

use super::{types::*, ActiveModel, BaseEntity, Column, Entity, RoleId};
use crate::{auth::AuthInfo, models::organization::OrganizationId, Error};

type QueryAs<'q, T> = sqlx::query::QueryAs<
    'q,
    sqlx::Postgres,
    T,
    <sqlx::Postgres as sqlx::database::HasArguments<'q>>::Arguments,
>;

/*
/// Get a Role from the database or return a `NotFound` error.
#[instrument(skip(db))]
pub async fn get(db: impl ConnectionTrait, auth: &AuthInfo, id: RoleId) -> Result<Model, error_stack::Report<Error>> {
    let object = super::Entity::new(auth)
        .find_by_id(id)?
        .one(&db)
        .await
        .change_context(Error::Db)?
        .ok_or(Error::NotFound(super::NAME))?;

    Ok(object)
}


*/
#[derive(Debug)]
struct OrderBy {
    column: Column,
    descending: bool,
}

impl OrderBy {
    fn as_order(&self) -> sea_orm::query::Order {
        if self.descending {
            sea_orm::query::Order::Desc
        } else {
            sea_orm::query::Order::Asc
        }
    }
}

impl std::str::FromStr for OrderBy {
    type Err = OrderByError;

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        let descending = field.starts_with('-');
        let field = if descending { &field[1..] } else { field };

        let column = match field {
            "updated_at" => Column::UpdatedAt,
            "created_at" => Column::CreatedAt,
            "name" => Column::Name,
            _ => return Err(OrderByError::InvalidField),
        };

        Ok(Self { column, descending })
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ListQueryFilters {
    pub page: Option<u32>,
    pub per_page: Option<u32>,

    pub order_by: Option<String>,
    #[serde(default)]
    pub id: Vec<RoleId>,
    pub updated_at_lte: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at_gte: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_lte: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_gte: Option<chrono::DateTime<chrono::Utc>>,
}

impl ListQueryFilters {
    pub fn apply(
        self,
        mut query: Select<BaseEntity>,
    ) -> Result<Select<BaseEntity>, error_stack::Report<Error>> {
        if !self.id.is_empty() {
            query = query.filter(Column::Id.is_in(self.id));
        }

        if let Some(value) = self.updated_at_lte {
            query = query.filter(Column::UpdatedAt.lt(value));
        }

        if let Some(value) = self.updated_at_gte {
            query = query.filter(Column::UpdatedAt.gt(value));
        }

        if let Some(value) = self.created_at_lte {
            query = query.filter(Column::CreatedAt.lt(value));
        }

        if let Some(value) = self.created_at_gte {
            query = query.filter(Column::CreatedAt.gt(value));
        }

        let order_by = OrderBy::from_str(self.order_by.as_deref().unwrap_or("name"))
            .change_context(Error::Filter)?;
        query = query.order_by(order_by.column, order_by.as_order());

        const MAX_PER_PAGE: u32 = 200;
        const DEFAULT_PER_PAGE: u32 = 50;
        let per_page = self
            .per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .min(MAX_PER_PAGE)
            .max(1);
        let offset = self.page.map(|p| (p * per_page) as u64);
        query = query.limit(per_page as u64).offset(offset);
        event!(Level::DEBUG, per_page, offset);

        Ok(query)
    }
}

/*
#[instrument(skip(db))]
pub async fn list(
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    filters: ListQueryFilters) -> Result<Vec<RoleListResult>, error_stack::Report<Error>> {

    let q = include_str!("list.sql");
    list_internal(q, db, auth, filters).await
}



async fn list_internal<T>(
    query_template: &str,
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    filters: &ListQueryFilters)
-> Result<Vec<T>, error_stack::Report<Error>>
where
    T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
{

    let order_direction = if descending { "DESC" } else { "ASC" };

    let q = query_template.replace("__insertion_point_order_by", &format!("{} {}", order_by_field.as_str(), order_direction));

        let q = q.replace("__insertion_point_filters", &filters.build_where_clause());

    let mut query = sqlx::query_as::<_, T>(q.as_str());

    let actor_ids = auth.actor_ids();
    event!(Level::DEBUG, organization_id=%auth.organization_id, actor_ids=?actor_ids);
    query = query

        .bind(&auth.organization_id)

        .bind(&actor_ids);

    query = filters.bind_to_query(query);

    let results = query
        .fetch_all(db)
        .await
        .change_context(Error::Db)?;

    Ok(results)
}

/// Create a new Role in the database.
pub async fn create(db: &mut PgConnection, auth: &AuthInfo, payload: RoleCreatePayload) -> Result<RoleCreateResult, error_stack::Report<Error>> {
    // TODO create permissions auth check

    let id = RoleId::new();

    create_raw(&mut *db, &id, &auth.organization_id, payload).await
}

/// Create a new Role in the database, allowing the ID to be explicitly specified
/// regardless of whether it would normally be allowed.
#[instrument(skip(db))]
pub async fn create_raw(
    db: &mut PgConnection,
    id: &RoleId,
    organization_id: &OrganizationId,
    payload: RoleCreatePayload
) -> Result<RoleCreateResult, error_stack::Report<Error>> {

    let result = query_file_as!(Role, "src/models/role/insert.sql",
        id.as_uuid(),
        organization_id.as_uuid(),
        &payload.name,payload.description.as_ref(),
        )
        .fetch_one(&mut *db)

        .await
        .change_context(Error::Db)?;




    Ok(result)
}



#[instrument(skip(db))]
pub async fn update(
    db: &mut PgConnection,
    auth: &AuthInfo,
    id: &RoleId,
    payload: RoleUpdatePayload)
-> Result<bool, error_stack::Report<Error>> {
    let actor_ids = auth.actor_ids();
    let result = query_file_scalar!("src/models/role/update.sql",
        id.as_uuid(),
        auth.organization_id.as_uuid(),
        &actor_ids,
        &payload.name as _,payload.description.as_ref() as _,
        )
        .fetch_optional(&mut *db)
        .await
        .change_context(Error::Db)?;

    let Some(is_owner) = result else {
        return Ok(false);
    };



    Ok(true)
}



#[instrument(skip(db))]
pub async fn delete(db: impl ConnectionTrait, auth: &AuthInfo, id: &RoleId) -> Result<bool, error_stack::Report<Error>> {
    let actor_ids = auth.actor_ids();
    let result = query_file!("src/models/role/delete.sql",
        id.as_uuid(),
        auth.organization_id.as_uuid(),
        &actor_ids
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
    Ok(result.rows_affected() > 0)
}

#[instrument(skip(db))]
pub async fn lookup_object_permissions(
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    #[allow(unused_variables)]
    id: &RoleId,
) -> Result<Option<ObjectPermission>, error_stack::Report<Error>> {
    let actor_ids = auth.actor_ids();
    let result = query_file_scalar!(
        "src/models/role/lookup_object_permissions.sql",
        auth.organization_id.as_uuid(),
        &actor_ids,

        )
        .fetch_one(db)
        .await
        .change_context(Error::Db)?;

    let perm = result.and_then(|r| ObjectPermission::from_str_infallible(&r));
    Ok(perm)
}
*/
