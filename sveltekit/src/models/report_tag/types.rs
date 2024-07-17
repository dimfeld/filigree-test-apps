#![allow(unused_imports, dead_code)]

use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use crate::models::{organization::OrganizationId, report::ReportId, tag::TagId};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct ReportTag {
    pub report_id: ReportId,
    pub tag_id: TagId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub type ReportTagListResult = ReportTag;

pub type ReportTagPopulatedGetResult = ReportTag;

pub type ReportTagPopulatedListResult = ReportTag;

pub type ReportTagCreateResult = ReportTag;

impl ReportTag {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_report_id() -> ReportId {
        <ReportId as Default>::default().into()
    }

    pub fn default_tag_id() -> TagId {
        <TagId as Default>::default().into()
    }

    pub fn default_organization_id() -> crate::models::organization::OrganizationId {
        <crate::models::organization::OrganizationId as Default>::default().into()
    }

    pub fn default_updated_at() -> chrono::DateTime<chrono::Utc> {
        <chrono::DateTime<chrono::Utc> as Default>::default().into()
    }

    pub fn default_created_at() -> chrono::DateTime<chrono::Utc> {
        <chrono::DateTime<chrono::Utc> as Default>::default().into()
    }
}

sqlx_json_decode!(ReportTag);

impl Default for ReportTag {
    fn default() -> Self {
        Self {
            report_id: Self::default_report_id(),
            tag_id: Self::default_tag_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReportTagCreatePayloadAndUpdatePayload {
    pub report_id: Option<ReportId>,
    pub tag_id: Option<TagId>,
}

pub type ReportTagCreatePayload = ReportTagCreatePayloadAndUpdatePayload;

pub type ReportTagUpdatePayload = ReportTagCreatePayloadAndUpdatePayload;

impl ReportTagCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_report_id() -> Option<ReportId> {
        None
    }

    pub fn default_tag_id() -> Option<TagId> {
        None
    }
}

impl Default for ReportTagCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            report_id: Self::default_report_id(),
            tag_id: Self::default_tag_id(),
        }
    }
}
