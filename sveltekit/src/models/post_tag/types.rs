#![allow(unused_imports, dead_code)]

use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use crate::models::{organization::OrganizationId, post::PostId, tag::TagId};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct PostTag {
    pub post_id: PostId,
    pub tag_id: TagId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub type PostTagListResult = PostTag;

pub type PostTagPopulatedGetResult = PostTag;

pub type PostTagPopulatedListResult = PostTag;

pub type PostTagCreateResult = PostTag;

impl PostTag {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
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

sqlx_json_decode!(PostTag);

impl Default for PostTag {
    fn default() -> Self {
        Self {
            post_id: Self::default_post_id(),
            tag_id: Self::default_tag_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct PostTagCreatePayloadAndUpdatePayload {
    pub post_id: Option<PostId>,
    pub tag_id: Option<TagId>,
}

pub type PostTagCreatePayload = PostTagCreatePayloadAndUpdatePayload;

pub type PostTagUpdatePayload = PostTagCreatePayloadAndUpdatePayload;

impl PostTagCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_post_id() -> Option<PostId> {
        None
    }

    pub fn default_tag_id() -> Option<TagId> {
        None
    }
}

impl Default for PostTagCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            post_id: Self::default_post_id(),
            tag_id: Self::default_tag_id(),
        }
    }
}
