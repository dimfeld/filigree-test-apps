#![allow(unused_imports, unused_variables, dead_code)]
use std::str::FromStr;

use error_stack::ResultExt;
use filigree::{
    auth::{AuthInfo as _, ObjectPermission},
    errors::OrderByError,
    sql::{BindingOperator, FilterBuilder, ValuesBuilder},
};
use serde::Deserialize;
use sqlx::{
    postgres::PgRow, query_file, query_file_as, query_file_scalar, PgConnection, PgExecutor,
};
use tracing::{event, instrument, Level};

use super::types::*;
use crate::{
    auth::AuthInfo,
    models::{organization::OrganizationId, report::ReportId, tag::TagId},
    Error,
};

#[derive(Debug, Default)]
enum OrderByField {
    #[default]
    UpdatedAt,
    CreatedAt,
}

impl OrderByField {
    fn as_str(&self) -> &str {
        match self {
            Self::UpdatedAt => "updated_at",
            Self::CreatedAt => "created_at",
        }
    }

    fn allowed_direction(&self, descending: bool) -> bool {
        match self {
            _ => true,
        }
    }
}

impl std::str::FromStr for OrderByField {
    type Err = OrderByError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s {
            "updated_at" => OrderByField::UpdatedAt,
            "created_at" => OrderByField::CreatedAt,
            _ => return Err(OrderByError::InvalidField),
        };

        Ok(value)
    }
}

fn parse_order_by(field: &str) -> Result<(bool, OrderByField), OrderByError> {
    let descending = field.starts_with('-');
    let field = if descending { &field[1..] } else { field };

    let value = OrderByField::from_str(field)?;
    if !value.allowed_direction(descending) {
        return Err(OrderByError::InvalidDirection);
    }
    Ok((descending, value))
}

#[derive(Deserialize, Debug, Default)]
pub struct ListQueryFilters {
    pub page: Option<u32>,
    pub per_page: Option<u32>,

    pub order_by: Option<String>,
    #[serde(default)]
    pub report_id: Vec<ReportId>,
    #[serde(default)]
    pub tag_id: Vec<TagId>,
    pub updated_at_lte: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at_gte: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_lte: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at_gte: Option<chrono::DateTime<chrono::Utc>>,
}

impl ListQueryFilters {
    fn build_where_clause(&self) -> String {
        let mut bindings = FilterBuilder::new(4);

        if !self.report_id.is_empty() {
            bindings.add_vec("report_id", &self.report_id);
        }

        if !self.tag_id.is_empty() {
            bindings.add_vec("tag_id", &self.tag_id);
        }

        if self.updated_at_lte.is_some() {
            bindings.add_option("updated_at", &self.updated_at_lte, BindingOperator::Lte);
        }

        if self.updated_at_gte.is_some() {
            bindings.add_option("updated_at", &self.updated_at_gte, BindingOperator::Gte);
        }

        if self.created_at_lte.is_some() {
            bindings.add_option("created_at", &self.created_at_lte, BindingOperator::Lte);
        }

        if self.created_at_gte.is_some() {
            bindings.add_option("created_at", &self.created_at_gte, BindingOperator::Gte);
        }

        let query = bindings.to_string();
        event!(Level::DEBUG, %query);
        query
    }

    fn bind_to_query<'a, T>(&'a self, mut query: QueryAs<'a, T>) -> QueryAs<'a, T> {
        if !self.report_id.is_empty() {
            event!(Level::DEBUG, report_id = ?self.report_id);
            query = query.bind(&self.report_id);
        }

        if !self.tag_id.is_empty() {
            event!(Level::DEBUG, tag_id = ?self.tag_id);
            query = query.bind(&self.tag_id);
        }

        if self.updated_at_lte.is_some() {
            event!(Level::DEBUG, updated_at_lte = ?self.updated_at_lte);
            query = query.bind(&self.updated_at_lte);
        }

        if self.updated_at_gte.is_some() {
            event!(Level::DEBUG, updated_at_gte = ?self.updated_at_gte);
            query = query.bind(&self.updated_at_gte);
        }

        if self.created_at_lte.is_some() {
            event!(Level::DEBUG, created_at_lte = ?self.created_at_lte);
            query = query.bind(&self.created_at_lte);
        }

        if self.created_at_gte.is_some() {
            event!(Level::DEBUG, created_at_gte = ?self.created_at_gte);
            query = query.bind(&self.created_at_gte);
        }

        query
    }
}

type QueryAs<'q, T> = sqlx::query::QueryAs<
    'q,
    sqlx::Postgres,
    T,
    <sqlx::Postgres as sqlx::database::HasArguments<'q>>::Arguments,
>;

impl ReportTag {
    fn check_missing_parent_error<T>(
        result: Result<T, sqlx::Error>,
    ) -> Result<T, error_stack::Report<Error>> {
        match result {
            Err(sqlx::Error::Database(e))
                if e.constraint() == Some("report_tags_report_id_fkey") =>
            {
                Err(e).change_context(Error::NotFound("Parent report_id"))
            }

            Err(sqlx::Error::Database(e)) if e.constraint() == Some("report_tags_tag_id_fkey") => {
                Err(e).change_context(Error::NotFound("Parent tag_id"))
            }

            _ => result.change_context(Error::Db),
        }
    }

    /// Get a ReportTag from the database
    #[instrument(skip(db))]
    pub async fn get(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        id: &(ReportId, TagId),
    ) -> Result<ReportTag, error_stack::Report<Error>> {
        auth.require_permission(super::READ_PERMISSION)?;

        let object = query_file_as!(
            ReportTag,
            "src/models/report_tag/select_one.sql",
            auth.organization_id.as_uuid(),
            id.0.as_uuid(),
            id.1.as_uuid()
        )
        .fetch_optional(db)
        .await
        .change_context(Error::Db)?
        .ok_or(Error::NotFound("ReportTag"))?;

        Ok(object)
    }

    #[instrument(skip(db))]
    pub async fn list(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        filters: &ListQueryFilters,
    ) -> Result<Vec<ReportTagListResult>, error_stack::Report<Error>> {
        let q = include_str!("list.sql");
        Self::list_internal(q, db, auth, filters).await
    }

    async fn list_internal<T>(
        query_template: &str,
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        filters: &ListQueryFilters,
    ) -> Result<Vec<T>, error_stack::Report<Error>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Send + Unpin,
    {
        auth.require_permission(super::READ_PERMISSION)?;

        const MAX_PER_PAGE: u32 = 200;
        const DEFAULT_PER_PAGE: u32 = 50;
        let per_page = filters
            .per_page
            .unwrap_or(DEFAULT_PER_PAGE)
            .min(MAX_PER_PAGE)
            .max(1) as i32;
        let offset = filters.page.unwrap_or(0) as i32 * per_page;
        event!(Level::DEBUG, per_page, offset);

        let (descending, order_by_field) =
            parse_order_by(filters.order_by.as_deref().unwrap_or("-updated_at"))
                .change_context(Error::Filter)?;
        let order_direction = if descending { "DESC" } else { "ASC" };

        let q = query_template.replace(
            "__insertion_point_order_by",
            &format!("{} {}", order_by_field.as_str(), order_direction),
        );

        let q = q.replace("__insertion_point_filters", &filters.build_where_clause());

        let mut query = sqlx::query_as::<_, T>(q.as_str());

        event!(Level::DEBUG, organization_id=%auth.organization_id);
        query = query
            .bind(&auth.organization_id)
            .bind(per_page)
            .bind(offset);

        query = filters.bind_to_query(query);

        let results = query.fetch_all(db).await.change_context(Error::Db)?;

        Ok(results)
    }

    /// Create a new ReportTag in the database.
    pub async fn create(
        db: &mut PgConnection,
        auth: &AuthInfo,
        payload: ReportTagCreatePayload,
    ) -> Result<ReportTagCreateResult, error_stack::Report<Error>> {
        auth.require_permission(super::CREATE_PERMISSION)?;

        let id = (
            payload
                .report_id
                .clone()
                .ok_or(Error::MissingId("report_id"))?,
            payload.tag_id.clone().ok_or(Error::MissingId("tag_id"))?,
        );

        Self::create_raw(&mut *db, &id, &auth.organization_id, payload).await
    }

    /// Create a new ReportTag in the database, allowing the ID to be explicitly specified
    /// regardless of whether it would normally be allowed.
    #[instrument(skip(db))]
    pub async fn create_raw(
        db: &mut PgConnection,
        id: &(ReportId, TagId),
        organization_id: &OrganizationId,
        payload: ReportTagCreatePayload,
    ) -> Result<ReportTagCreateResult, error_stack::Report<Error>> {
        let result = query_file_as!(
            ReportTag,
            "src/models/report_tag/insert.sql",
            id.0.as_uuid(),
            id.1.as_uuid(),
            organization_id.as_uuid()
        )
        .fetch_one(&mut *db)
        .await;

        let result = Self::check_missing_parent_error(result)?;

        Ok(result)
    }

    #[instrument(skip(db))]
    pub async fn update(
        db: &mut PgConnection,
        auth: &AuthInfo,
        id: &(ReportId, TagId),
        payload: ReportTagUpdatePayload,
    ) -> Result<bool, error_stack::Report<Error>> {
        auth.require_permission(super::WRITE_PERMISSION)?;

        let result = query_file_scalar!(
            "src/models/report_tag/update.sql",
            id.0.as_uuid(),
            id.1.as_uuid(),
            auth.organization_id.as_uuid()
        )
        .execute(&mut *db)
        .await
        .change_context(Error::Db)?;

        if result.rows_affected() == 0 {
            return Ok(false);
        }

        Ok(true)
    }

    #[instrument(skip(db))]
    pub async fn delete(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        id: &(ReportId, TagId),
    ) -> Result<bool, error_stack::Report<Error>> {
        auth.require_permission(super::CREATE_PERMISSION)?;

        let result = query_file!(
            "src/models/report_tag/delete.sql",
            id.0.as_uuid(),
            id.1.as_uuid(),
            auth.organization_id.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
        Ok(result.rows_affected() > 0)
    }

    #[instrument(skip(db))]
    pub async fn lookup_object_permissions(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        #[allow(unused_variables)] id: &(ReportId, TagId),
    ) -> Result<Option<ObjectPermission>, error_stack::Report<Error>> {
        let mut saw_write = false;
        let mut saw_read = false;

        use super::{OWNER_PERMISSION, READ_PERMISSION, WRITE_PERMISSION};

        for perm in &auth.permissions {
            if perm == OWNER_PERMISSION {
                return Ok(Some(ObjectPermission::Owner));
            } else if perm == WRITE_PERMISSION {
                saw_write = true;
            } else if perm == READ_PERMISSION {
                saw_read = true;
            }
        }

        if saw_write {
            return Ok(Some(ObjectPermission::Write));
        } else if saw_read {
            return Ok(Some(ObjectPermission::Read));
        } else {
            return Ok(None);
        }
    }

    /// Update or insert a child of the parent report_id.

    #[instrument(skip(db))]
    pub async fn upsert_with_parent_report(
        db: impl PgExecutor<'_>,
        organization_id: &OrganizationId,
        parent_id: &ReportId,
        payload: &ReportTagUpdatePayload,
    ) -> Result<ReportTag, error_stack::Report<Error>> {
        let id = (
            payload
                .report_id
                .clone()
                .ok_or(Error::MissingId("report_id"))?,
            payload.tag_id.clone().ok_or(Error::MissingId("tag_id"))?,
        );

        let result = query_file_as!(
            ReportTag,
            "src/models/report_tag/upsert_single_child_of_report.sql",
            id.0.as_uuid(),
            id.1.as_uuid(),
            organization_id.as_uuid()
        )
        .fetch_one(db)
        .await;
        Self::check_missing_parent_error(result)
    }

    /// Update the children of the given parent.
    /// Insert new values that are not yet in the database and
    /// delete existing values that are not in the payload.
    #[instrument(skip(db))]
    pub async fn update_all_with_parent_report(
        db: &mut PgConnection,
        organization_id: &OrganizationId,
        parent_id: &ReportId,
        payload: &[ReportTagUpdatePayload],
    ) -> Result<Vec<ReportTag>, error_stack::Report<Error>> {
        if payload.is_empty() {
            Self::delete_all_children_of_report(db, organization_id, parent_id).await?;
            Ok(Vec::new())
        } else {
            // First, we upsert the existing children.

            let q = include_str!("upsert_children_of_report.sql");
            let bindings = ValuesBuilder {
                first_parameter: 1,
                num_values: payload.len(),
                num_columns: 2 + 1 + 0,
            };
            let q = q.replace("__insertion_point_insert_values", &bindings.to_string());

            let mut query = sqlx::query_as::<_, ReportTag>(q.as_str())
                .bind(organization_id)
                .bind(parent_id);

            for p in payload {
                let report_id = parent_id;
                let tag_id = p.tag_id.as_ref().ok_or(Error::MissingId("tag_id"))?;

                query = query.bind(report_id).bind(tag_id).bind(organization_id)
            }

            let results = query.fetch_all(&mut *db).await;
            let results = Self::check_missing_parent_error(results)?;

            // Delete any of the children that were not sent in.
            let ids = results
                .iter()
                .map(|o| o.tag_id.as_uuid().clone())
                .collect::<Vec<_>>();

            query_file!(
                "src/models/report_tag/delete_removed_children_of_report.sql",
                organization_id.as_uuid(),
                parent_id.as_uuid(),
                &ids
            )
            .execute(db)
            .await
            .change_context(Error::Db)?;

            Ok(results)
        }
    }

    /// Delete a child object, making sure that its parent ID matches.
    #[instrument(skip(db))]
    pub async fn delete_with_parent_report(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        parent_id: &ReportId,
        id: &(ReportId, TagId),
    ) -> Result<bool, error_stack::Report<Error>> {
        let result = query_file!(
            "src/models/report_tag/delete_with_parent_report.sql",
            auth.organization_id.as_uuid(),
            parent_id.as_uuid(),
            id.0.as_uuid(),
            id.1.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
        Ok(result.rows_affected() > 0)
    }

    /// Delete all children of the given parent. This function does not do permissions checks.
    #[instrument(skip(db))]
    pub async fn delete_all_children_of_report(
        db: impl PgExecutor<'_>,
        organization_id: &OrganizationId,
        parent_id: &ReportId,
    ) -> Result<bool, error_stack::Report<Error>> {
        let result = query_file!(
            "src/models/report_tag/delete_all_children_of_report.sql",
            organization_id.as_uuid(),
            parent_id.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
        Ok(result.rows_affected() > 0)
    }

    /// Update or insert a child of the parent tag_id.

    #[instrument(skip(db))]
    pub async fn upsert_with_parent_tag(
        db: impl PgExecutor<'_>,
        organization_id: &OrganizationId,
        parent_id: &TagId,
        payload: &ReportTagUpdatePayload,
    ) -> Result<ReportTag, error_stack::Report<Error>> {
        let id = (
            payload
                .report_id
                .clone()
                .ok_or(Error::MissingId("report_id"))?,
            payload.tag_id.clone().ok_or(Error::MissingId("tag_id"))?,
        );

        let result = query_file_as!(
            ReportTag,
            "src/models/report_tag/upsert_single_child_of_tag.sql",
            id.0.as_uuid(),
            id.1.as_uuid(),
            organization_id.as_uuid()
        )
        .fetch_one(db)
        .await;
        Self::check_missing_parent_error(result)
    }

    /// Update the children of the given parent.
    /// Insert new values that are not yet in the database and
    /// delete existing values that are not in the payload.
    #[instrument(skip(db))]
    pub async fn update_all_with_parent_tag(
        db: &mut PgConnection,
        organization_id: &OrganizationId,
        parent_id: &TagId,
        payload: &[ReportTagUpdatePayload],
    ) -> Result<Vec<ReportTag>, error_stack::Report<Error>> {
        if payload.is_empty() {
            Self::delete_all_children_of_tag(db, organization_id, parent_id).await?;
            Ok(Vec::new())
        } else {
            // First, we upsert the existing children.

            let q = include_str!("upsert_children_of_tag.sql");
            let bindings = ValuesBuilder {
                first_parameter: 1,
                num_values: payload.len(),
                num_columns: 2 + 1 + 0,
            };
            let q = q.replace("__insertion_point_insert_values", &bindings.to_string());

            let mut query = sqlx::query_as::<_, ReportTag>(q.as_str())
                .bind(organization_id)
                .bind(parent_id);

            for p in payload {
                let report_id = p.report_id.as_ref().ok_or(Error::MissingId("report_id"))?;
                let tag_id = parent_id;

                query = query.bind(report_id).bind(tag_id).bind(organization_id)
            }

            let results = query.fetch_all(&mut *db).await;
            let results = Self::check_missing_parent_error(results)?;

            // Delete any of the children that were not sent in.
            let ids = results
                .iter()
                .map(|o| o.report_id.as_uuid().clone())
                .collect::<Vec<_>>();

            query_file!(
                "src/models/report_tag/delete_removed_children_of_tag.sql",
                organization_id.as_uuid(),
                parent_id.as_uuid(),
                &ids
            )
            .execute(db)
            .await
            .change_context(Error::Db)?;

            Ok(results)
        }
    }

    /// Delete a child object, making sure that its parent ID matches.
    #[instrument(skip(db))]
    pub async fn delete_with_parent_tag(
        db: impl PgExecutor<'_>,
        auth: &AuthInfo,
        parent_id: &TagId,
        id: &(ReportId, TagId),
    ) -> Result<bool, error_stack::Report<Error>> {
        let result = query_file!(
            "src/models/report_tag/delete_with_parent_tag.sql",
            auth.organization_id.as_uuid(),
            parent_id.as_uuid(),
            id.0.as_uuid(),
            id.1.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
        Ok(result.rows_affected() > 0)
    }

    /// Delete all children of the given parent. This function does not do permissions checks.
    #[instrument(skip(db))]
    pub async fn delete_all_children_of_tag(
        db: impl PgExecutor<'_>,
        organization_id: &OrganizationId,
        parent_id: &TagId,
    ) -> Result<bool, error_stack::Report<Error>> {
        let result = query_file!(
            "src/models/report_tag/delete_all_children_of_tag.sql",
            organization_id.as_uuid(),
            parent_id.as_uuid()
        )
        .execute(db)
        .await
        .change_context(Error::Db)?;
        Ok(result.rows_affected() > 0)
    }
}
