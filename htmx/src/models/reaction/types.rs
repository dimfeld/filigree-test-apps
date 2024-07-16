#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::ReactionId;
use crate::models::{organization::OrganizationId, post::PostId};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct Reaction {
    pub id: ReactionId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub typ: String,
    pub post_id: PostId,
}

pub type ReactionListResult = Reaction;

pub type ReactionPopulatedGetResult = Reaction;

pub type ReactionPopulatedListResult = Reaction;

pub type ReactionCreateResult = Reaction;

impl Reaction {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> ReactionId {
        <ReactionId as Default>::default().into()
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

    pub fn default_typ() -> String {
        <String as Default>::default().into()
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

sqlx_json_decode!(Reaction);

impl Default for Reaction {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            typ: Self::default_typ(),
            post_id: Self::default_post_id(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReactionCreatePayloadAndUpdatePayload {
    pub id: Option<ReactionId>,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub typ: String,
    pub post_id: PostId,
}

pub type ReactionCreatePayload = ReactionCreatePayloadAndUpdatePayload;

pub type ReactionUpdatePayload = ReactionCreatePayloadAndUpdatePayload;

impl ReactionCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<ReactionId> {
        None
    }

    pub fn default_typ() -> String {
        <String as Default>::default().into()
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

impl Default for ReactionCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            typ: Self::default_typ(),
            post_id: Self::default_post_id(),
        }
    }
}
