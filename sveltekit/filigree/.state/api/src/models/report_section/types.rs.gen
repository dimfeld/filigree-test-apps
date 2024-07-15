#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use sea_orm::prelude::*;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{entity::ActiveModel, ReportSectionId};
use crate::models::{organization::OrganizationId, report::ReportId};

pub type Model = super::entity::Model;

pub type ReportSectionUserView = Model;

pub type ReportSectionOwnerView = Model;

pub type ReportSectionListResult = Model;

pub type ReportSectionCreateResult = Model;

pub type ReportSection = Model;

impl Model {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> ReportSectionId {
        <ReportSectionId as Default>::default().into()
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

    pub fn default_name() -> String {
        <String as Default>::default().into()
    }

    pub fn default_viz() -> String {
        <String as Default>::default().into()
    }

    pub fn default_options() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_report_id() -> ReportId {
        <ReportId as Default>::default().into()
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
            name: Self::default_name(),
            viz: Self::default_viz(),
            options: Self::default_options(),
            report_id: Self::default_report_id(),
        }
    }
}
