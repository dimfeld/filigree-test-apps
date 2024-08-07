#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::CommentId;
use crate::models::{organization::OrganizationId, post::PostId};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct Comment {
    pub id: CommentId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub body: String,
    pub post_id: PostId,
}

pub type CommentListResult = Comment;

pub type CommentPopulatedGetResult = Comment;

pub type CommentPopulatedListResult = Comment;

pub type CommentCreateResult = Comment;

impl Comment {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> CommentId {
        <CommentId as Default>::default().into()
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

    pub fn default_body() -> String {
        <String as Default>::default().into()
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

sqlx_json_decode!(Comment);

impl Default for Comment {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            body: Self::default_body(),
            post_id: Self::default_post_id(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct CommentCreatePayloadAndUpdatePayload {
    pub id: Option<CommentId>,
    pub body: String,
    pub post_id: PostId,
}

pub type CommentCreatePayload = CommentCreatePayloadAndUpdatePayload;

pub type CommentUpdatePayload = CommentCreatePayloadAndUpdatePayload;

impl CommentCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<CommentId> {
        None
    }

    pub fn default_body() -> String {
        <String as Default>::default().into()
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

impl Default for CommentCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            body: Self::default_body(),
            post_id: Self::default_post_id(),
        }
    }
}
