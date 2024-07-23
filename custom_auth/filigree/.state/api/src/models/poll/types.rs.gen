#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::PollId;
use crate::models::{organization::OrganizationId, post::PostId};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct Poll {
    pub id: PollId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub question: String,
    pub answers: serde_json::Value,
    pub post_id: PostId,
}

pub type PollListResult = Poll;

pub type PollPopulatedGetResult = Poll;

pub type PollPopulatedListResult = Poll;

pub type PollCreateResult = Poll;

impl Poll {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> PollId {
        <PollId as Default>::default().into()
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

    pub fn default_question() -> String {
        <String as Default>::default().into()
    }

    pub fn default_answers() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

sqlx_json_decode!(Poll);

impl Default for Poll {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            question: Self::default_question(),
            answers: Self::default_answers(),
            post_id: Self::default_post_id(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct PollCreatePayloadAndUpdatePayload {
    pub id: Option<PollId>,
    pub question: String,
    pub answers: serde_json::Value,
    pub post_id: PostId,
}

pub type PollCreatePayload = PollCreatePayloadAndUpdatePayload;

pub type PollUpdatePayload = PollCreatePayloadAndUpdatePayload;

impl PollCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<PollId> {
        None
    }

    pub fn default_question() -> String {
        <String as Default>::default().into()
    }

    pub fn default_answers() -> serde_json::Value {
        <serde_json::Value as Default>::default().into()
    }

    pub fn default_post_id() -> PostId {
        <PostId as Default>::default().into()
    }
}

impl Default for PollCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            question: Self::default_question(),
            answers: Self::default_answers(),
            post_id: Self::default_post_id(),
        }
    }
}
