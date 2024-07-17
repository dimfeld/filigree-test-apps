#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::ReportId;
use crate::models::{
    organization::OrganizationId,
    report_section::{
        ReportSection, ReportSectionCreatePayload, ReportSectionCreateResult, ReportSectionId,
        ReportSectionUpdatePayload,
    },
    report_tag::{ReportTag, ReportTagCreatePayload, ReportTagUpdatePayload},
    tag::{Tag, TagCreatePayload, TagCreateResult, TagId, TagUpdatePayload},
};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct Report {
    pub id: ReportId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
}

pub type ReportListResult = Report;

impl Report {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> ReportId {
        <ReportId as Default>::default().into()
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

    pub fn default_title() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }

    pub fn default_ui() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }
}

sqlx_json_decode!(Report);

impl Default for Report {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            title: Self::default_title(),
            description: Self::default_description(),
            ui: Self::default_ui(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReportCreatePayload {
    pub id: Option<ReportId>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
    pub report_sections: Option<Vec<ReportSectionCreatePayload>>,
    pub tags: Option<Vec<TagId>>,
}

impl ReportCreatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<ReportId> {
        None
    }

    pub fn default_title() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }

    pub fn default_ui() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_report_sections() -> Option<Vec<ReportSectionCreatePayload>> {
        None
    }

    pub fn default_tags() -> Option<Vec<TagId>> {
        None
    }
}

impl Default for ReportCreatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            title: Self::default_title(),
            description: Self::default_description(),
            ui: Self::default_ui(),
            report_sections: Self::default_report_sections(),
            tags: Self::default_tags(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct ReportCreateResult {
    pub id: ReportId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
    pub report_sections: Vec<ReportSection>,
    pub tags: Vec<TagId>,
}

impl ReportCreateResult {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> ReportId {
        <ReportId as Default>::default().into()
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

    pub fn default_title() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }

    pub fn default_ui() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_report_sections() -> Vec<ReportSection> {
        <Vec<ReportSection> as Default>::default().into()
    }

    pub fn default_tags() -> Vec<TagId> {
        <Vec<TagId> as Default>::default().into()
    }
}

impl Default for ReportCreateResult {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            title: Self::default_title(),
            description: Self::default_description(),
            ui: Self::default_ui(),
            report_sections: Self::default_report_sections(),
            tags: Self::default_tags(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct ReportPopulatedGetResult {
    pub id: ReportId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
    pub report_sections: Vec<ReportSection>,
    pub tags: Vec<Tag>,
}

impl ReportPopulatedGetResult {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> ReportId {
        <ReportId as Default>::default().into()
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

    pub fn default_title() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }

    pub fn default_ui() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_report_sections() -> Vec<ReportSection> {
        <Vec<ReportSection> as Default>::default().into()
    }

    pub fn default_tags() -> Vec<Tag> {
        <Vec<Tag> as Default>::default().into()
    }
}

sqlx_json_decode!(ReportPopulatedGetResult);

impl Default for ReportPopulatedGetResult {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            title: Self::default_title(),
            description: Self::default_description(),
            ui: Self::default_ui(),
            report_sections: Self::default_report_sections(),
            tags: Self::default_tags(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct ReportPopulatedListResult {
    pub id: ReportId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
    pub report_section_ids: Vec<ReportSectionId>,
    pub tag_ids: Vec<TagId>,
}

impl ReportPopulatedListResult {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> ReportId {
        <ReportId as Default>::default().into()
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

    pub fn default_title() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }

    pub fn default_ui() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_report_section_ids() -> Vec<ReportSectionId> {
        <Vec<ReportSectionId> as Default>::default().into()
    }

    pub fn default_tag_ids() -> Vec<TagId> {
        <Vec<TagId> as Default>::default().into()
    }
}

sqlx_json_decode!(ReportPopulatedListResult);

impl Default for ReportPopulatedListResult {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            title: Self::default_title(),
            description: Self::default_description(),
            ui: Self::default_ui(),
            report_section_ids: Self::default_report_section_ids(),
            tag_ids: Self::default_tag_ids(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReportUpdatePayload {
    pub id: Option<ReportId>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
    pub report_sections: Option<Vec<ReportSectionUpdatePayload>>,
    pub tags: Option<Vec<TagId>>,
}

impl ReportUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<ReportId> {
        None
    }

    pub fn default_title() -> String {
        <String as Default>::default().into()
    }

    pub fn default_description() -> Option<String> {
        None
    }

    pub fn default_ui() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_report_sections() -> Option<Vec<ReportSectionUpdatePayload>> {
        None
    }

    pub fn default_tags() -> Option<Vec<TagId>> {
        None
    }
}

impl Default for ReportUpdatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            title: Self::default_title(),
            description: Self::default_description(),
            ui: Self::default_ui(),
            report_sections: Self::default_report_sections(),
            tags: Self::default_tags(),
        }
    }
}
