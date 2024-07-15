#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use sea_orm::prelude::*;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{entity::ActiveModel, PostImageId};
use crate::models::{organization::OrganizationId, post::PostId};

pub type Model = super::entity::Model;

pub type PostImageListResult = Model;

pub type PostImageCreateResult = Model;

pub type PostImage = Model;

impl Model {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> PostImageId {
        <PostImageId as Default>::default().into()
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

    pub fn default_file_storage_key() -> String {
        <String as Default>::default().into()
    }

    pub fn default_file_storage_bucket() -> String {
        <String as Default>::default().into()
    }

    pub fn default_file_original_name() -> Option<String> {
        None
    }

    pub fn default_file_size() -> Option<i64> {
        None
    }

    pub fn default_file_hash() -> Option<Vec<u8>> {
        None
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
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
            file_storage_key: Self::default_file_storage_key(),
            file_storage_bucket: Self::default_file_storage_bucket(),
            file_original_name: Self::default_file_original_name(),
            file_size: Self::default_file_size(),
            file_hash: Self::default_file_hash(),
            post_id: Self::default_post_id(),
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
pub struct PostImageUserViewAndOwnerView {
    pub id: PostImageId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub file_size: Option<i64>,
    pub file_hash: Option<Vec<u8>>,
    pub post_id: PostId,
}

pub type PostImageUserView = PostImageUserViewAndOwnerView;

pub type PostImageOwnerView = PostImageUserViewAndOwnerView;

impl PostImageUserViewAndOwnerView {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> PostImageId {
        <PostImageId as Default>::default().into()
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

    pub fn default_file_size() -> Option<i64> {
        None
    }

    pub fn default_file_hash() -> Option<Vec<u8>> {
        None
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

sqlx_json_decode!(PostImageUserViewAndOwnerView);

impl Default for PostImageUserViewAndOwnerView {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            file_size: Self::default_file_size(),
            file_hash: Self::default_file_hash(),
            post_id: Self::default_post_id(),
        }
    }
}
