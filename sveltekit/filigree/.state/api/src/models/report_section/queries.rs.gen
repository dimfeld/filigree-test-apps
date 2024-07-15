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

use super::{types::*, ActiveModel, BaseEntity, Column, Entity, ReportSectionId};
use crate::{
    auth::AuthInfo,
    models::{organization::OrganizationId, report::ReportId},
    Error,
};

type QueryAs<'q, T> = sqlx::query::QueryAs<
    'q,
    sqlx::Postgres,
    T,
    <sqlx::Postgres as sqlx::database::HasArguments<'q>>::Arguments,
>;

fn check_missing_parent_error<T>(
    result: Result<T, DbErr>,
) -> Result<T, error_stack::Report<Error>> {
    match result {
        Err(DbErr::Exec(sea_orm::error::RuntimeErr::SqlxError(sqlx::Error::Database(e))))
            if e.constraint() == Some("report_sections_report_id_fkey") =>
        {
            Err(e).change_context(Error::NotFound("Parent Report"))
        }
        _ => result.change_context(Error::Db),
    }
}

/*
/// Get a ReportSection from the database or return a `NotFound` error.
#[instrument(skip(db))]
pub async fn get(db: impl ConnectionTrait, auth: &AuthInfo, id: ReportSectionId) -> Result<Model, error_stack::Report<Error>> {
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
    pub id: Vec<ReportSectionId>,
    #[serde(default)]
    pub report_id: Vec<ReportId>,
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

        if !self.report_id.is_empty() {
            query = query.filter(Column::ReportId.is_in(self.report_id));
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

        let order_by = OrderBy::from_str(self.order_by.as_deref().unwrap_or("-updated_at"))
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
    filters: ListQueryFilters) -> Result<Vec<ReportSectionListResult>, error_stack::Report<Error>> {

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

/// Create a new ReportSection in the database.
pub async fn create(db: &mut PgConnection, auth: &AuthInfo, payload: ReportSectionCreatePayload) -> Result<ReportSectionCreateResult, error_stack::Report<Error>> {
    // TODO create permissions auth check

    let id = ReportSectionId::new();

    create_raw(&mut *db, &id, &auth.organization_id, payload).await
}

/// Create a new ReportSection in the database, allowing the ID to be explicitly specified
/// regardless of whether it would normally be allowed.
#[instrument(skip(db))]
pub async fn create_raw(
    db: &mut PgConnection,
    id: &ReportSectionId,
    organization_id: &OrganizationId,
    payload: ReportSectionCreatePayload
) -> Result<ReportSectionCreateResult, error_stack::Report<Error>> {

    let result = query_file_as!(ReportSection, "src/models/report_section/insert.sql",
        id.as_uuid(),
        organization_id.as_uuid(),
        &payload.name,&payload.viz,&payload.options,&payload.report_id as _,
        )
        .fetch_one(&mut *db)

        .await;

    let result = check_missing_parent_error(result)?;





    Ok(result)
}



#[instrument(skip(db))]
pub async fn update(
    db: &mut PgConnection,
    auth: &AuthInfo,
    id: &ReportSectionId,
    payload: ReportSectionUpdatePayload)
-> Result<bool, error_stack::Report<Error>> {
    let actor_ids = auth.actor_ids();
    let result = query_file_scalar!("src/models/report_section/update.sql",
        id.as_uuid(),
        auth.organization_id.as_uuid(),
        &actor_ids,
        &payload.name as _,&payload.viz as _,&payload.options as _,&payload.report_id as _,
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
pub async fn delete(db: impl ConnectionTrait, auth: &AuthInfo, id: &ReportSectionId) -> Result<bool, error_stack::Report<Error>> {
    let actor_ids = auth.actor_ids();
    let result = query_file!("src/models/report_section/delete.sql",
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
    id: &ReportSectionId,
) -> Result<Option<ObjectPermission>, error_stack::Report<Error>> {
    let actor_ids = auth.actor_ids();
    let result = query_file_scalar!(
        "src/models/report_section/lookup_object_permissions.sql",
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

/// Update or insert a child of the given parent.

#[instrument(skip(db))]
pub async fn upsert_with_parent(
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    is_owner: bool,
    parent_id: &ReportId,
    payload: ActiveModel,
) -> Result<ReportSection, error_stack::Report<Error>> {
    todo!();
    /*
        let id = payload.id.clone().unwrap_or_else(ReportSectionId::new);
        let result = query_file_as!(ReportSection, "src/models/report_section/upsert_single_child.sql",

            id.as_uuid(),
            organization_id.as_uuid(),
            &payload.name,&payload.viz,&payload.options,&payload.report_id as _,
            )
            .fetch_one(db)
            .await;
        check_missing_parent_error(result)
    */
}

/*
/// Update a single child of the given parent. This does nothing if the child doesn't exist.
#[instrument(skip(db))]
pub async fn update_one_with_parent(
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    is_owner: bool,
    parent_id: &ReportId,
    id: &ReportSectionId,
    mut payload: ReportSectionUpdatePayload
    ) -> Result<bool, error_stack::Report<Error>> {

    todo!();
    payload.report_id = parent_id.clone();

    let actor_ids = auth.actor_ids();
    let result = query_file!("src/models/report_section/update_one_with_parent.sql",
        id.as_uuid(),
        parent_id.as_uuid(),
        auth.organization_id.as_uuid(),
        &actor_ids,
        &payload.name as _,&payload.viz as _,&payload.options as _,
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;

    Ok(result.rows_affected() > 0)
}
*/

/// Update the children of the given parent.
/// Insert new values that are not yet in the database and
/// delete existing values that are not in the payload.
#[instrument(skip(db))]
pub async fn update_all_with_parent(
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    parent_id: ReportId,
    mut payload: Vec<ActiveModel>,
) -> Result<Vec<ReportSection>, error_stack::Report<Error>> {
    if payload.is_empty() {
        Entity::new(auth)
            .delete_many()?
            .filter(Column::ReportId.eq(parent_id))
            .exec(&db)
            .await
            .change_context(Error::Db)?;

        Ok(Vec::new())
    } else {
        let parent_entity = crate::models::report::Entity::new(auth);

        let update_columns = &[Column::Name, Column::Viz, Column::Options];

        for model in payload.iter_mut() {
            if model.report_id.is_set() {
                model.report_id = ActiveValue::Unchanged(parent_id.clone());
            }

            if model.id.is_not_set() {
                model.id = ActiveValue::set(ReportSectionId::new());
            }
        }

        // Upsert existing children
        let results = Entity::new(auth)
            .insert_many(payload)?
            .on_conflict(
                sea_query::OnConflict::column(Column::Id)
                    .update_columns(update_columns.to_owned())
                    .value(Column::UpdatedAt, Expr::cust("now()"))
                    .action_and_where(Column::ReportId.eq(parent_id))
                    .to_owned(),
            )
            .exec_with_returning(&db)
            .await;
        let results = check_missing_parent_error(results)?;

        // Delete any of the children that were not sent in.
        let ids = results.iter().map(|o| o.id.clone()).collect::<Vec<_>>();
        Entity::new(auth)
            .delete_many()?
            .filter(Column::ReportId.eq(parent_id))
            .filter(Column::Id.is_not_in(ids))
            .exec(&db)
            .await
            .change_context(Error::Db)?;

        Ok(results)
    }
}

/*
/// Delete a child object, making sure that its parent ID matches.
#[instrument(skip(db))]
pub async fn delete_with_parent(
    db: impl ConnectionTrait,
    auth: &AuthInfo,
    parent_id: &ReportId,
    child_id: &ReportSectionId)
    -> Result<bool, error_stack::Report<Error>> {

    let result = query_file!("src/models/report_section/delete_with_parent.sql",

        auth.organization_id.as_uuid(),

        parent_id.as_uuid(),
        child_id.as_uuid(),
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
    Ok(result.rows_affected() > 0)
}

/// Delete all children of the given parent. This function does not do permissions checks.
#[instrument(skip(db))]
pub async fn delete_all_children_of_parent(
    db: impl PgExecutor<'_>,
    organization_id: &OrganizationId,
    parent_id: &ReportId)
    -> Result<bool, error_stack::Report<Error>> {

    let result = query_file!("src/models/report_section/delete_all_children.sql",

        organization_id.as_uuid(),

        parent_id.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
    Ok(result.rows_affected() > 0)
}
*/
