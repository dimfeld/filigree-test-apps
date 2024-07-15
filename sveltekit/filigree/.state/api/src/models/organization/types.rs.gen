#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use sea_orm::prelude::*;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{entity::ActiveModel, OrganizationId};

pub type Model = super::entity::Model;

pub type OrganizationListResult = Model;

pub type OrganizationCreateResult = Model;

pub type Organization = Model;

impl Model {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> OrganizationId {
        <OrganizationId as Default>::default().into()
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

    pub fn default_owner() -> Option<crate::models::user::UserId> {
        None
    }

    pub fn default_default_role() -> Option<crate::models::role::RoleId> {
        None
    }

    pub fn default_active() -> bool {
        <bool as Default>::default().into()
    }
}

sqlx_json_decode!(Model);

impl Default for Model {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            name: Self::default_name(),
            owner: Self::default_owner(),
            default_role: Self::default_default_role(),
            active: Self::default_active(),
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
    sea_orm::DeriveIntoActiveModel,
    sea_orm::FromQueryResult,
)]
pub struct OrganizationOwnerView {
    pub id: OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub owner: Option<crate::models::user::UserId>,
    pub default_role: Option<crate::models::role::RoleId>,
}

impl OrganizationOwnerView {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> OrganizationId {
        <OrganizationId as Default>::default().into()
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

    pub fn default_owner() -> Option<crate::models::user::UserId> {
        None
    }

    pub fn default_default_role() -> Option<crate::models::role::RoleId> {
        None
    }
}

sqlx_json_decode!(OrganizationOwnerView);

impl Default for OrganizationOwnerView {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            name: Self::default_name(),
            owner: Self::default_owner(),
            default_role: Self::default_default_role(),
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
    sea_orm::DeriveIntoActiveModel,
    sea_orm::FromQueryResult,
)]
pub struct OrganizationUserView {
    pub id: OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
}

impl OrganizationUserView {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> OrganizationId {
        <OrganizationId as Default>::default().into()
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
}

sqlx_json_decode!(OrganizationUserView);

impl Default for OrganizationUserView {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            name: Self::default_name(),
        }
    }
}
