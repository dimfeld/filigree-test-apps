#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use sea_orm::prelude::*;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{entity::ActiveModel, UserId};
use crate::models::organization::OrganizationId;

pub type Model = super::entity::Model;

pub type UserUserView = Model;

pub type UserOwnerView = Model;

pub type UserListResult = Model;

pub type UserCreateResult = Model;

pub type User = Model;

impl Model {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> UserId {
        <UserId as Default>::default().into()
    }

    pub fn default_organization_id() -> Option<crate::models::organization::OrganizationId> {
        None
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

    pub fn default_email() -> Option<String> {
        None
    }

    pub fn default_avatar_url() -> Option<String> {
        None
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
            email: Self::default_email(),
            avatar_url: Self::default_avatar_url(),
        }
    }
}
