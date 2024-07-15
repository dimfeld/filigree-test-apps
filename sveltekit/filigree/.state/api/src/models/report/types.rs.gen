#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use sea_orm::prelude::*;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{entity::ActiveModel, ReportId};
use crate::models::{
    organization::OrganizationId,
    report_section::{
        ReportSection, ReportSectionCreatePayload, ReportSectionId, ReportSectionUpdatePayload,
    },
};

pub type Model = super::entity::Model;

pub type ReportUserView = Model;

pub type ReportOwnerView = Model;

pub type ReportListResult = Model;

pub type Report = Model;

impl Model {
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

sqlx_json_decode!(Model);

impl Default for Model {
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

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    schemars::JsonSchema,
    sqlx::FromRow,
    sea_orm::FromQueryResult,
)]
pub struct ReportCreateResult {
    pub id: ReportId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub title: String,
    pub description: Option<String>,
    pub ui: serde_json::Value,
    pub report_sections: Vec<ReportSection>,
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
        }
    }
}
