#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::PostId;
use crate::models::{
    comment::{Comment, CommentCreatePayload, CommentId, CommentUpdatePayload},
    organization::OrganizationId,
    poll::{Poll, PollCreatePayload, PollId, PollUpdatePayload},
    post_image::{PostImage, PostImageCreatePayload, PostImageId, PostImageUpdatePayload},
    reaction::{Reaction, ReactionCreatePayload, ReactionId, ReactionUpdatePayload},
};

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct Post {
    pub id: PostId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub subject: String,
    pub body: String,
}

pub type PostListResult = Post;

pub type PostCreateResult = Post;

impl Post {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> PostId {
        <PostId as Default>::default().into()
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

    pub fn default_subject() -> String {
        <String as Default>::default().into()
    }

    pub fn default_body() -> String {
        <String as Default>::default().into()
    }
}

sqlx_json_decode!(Post);

impl Default for Post {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            subject: Self::default_subject(),
            body: Self::default_body(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow)]
#[cfg_attr(test, derive(Serialize))]
pub struct PostCreatePayloadAndUpdatePayload {
    pub id: Option<PostId>,
    pub subject: String,
    pub body: String,
}

pub type PostCreatePayload = PostCreatePayloadAndUpdatePayload;

pub type PostUpdatePayload = PostCreatePayloadAndUpdatePayload;

impl PostCreatePayloadAndUpdatePayload {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> Option<PostId> {
        None
    }

    pub fn default_subject() -> String {
        <String as Default>::default().into()
    }

    pub fn default_body() -> String {
        <String as Default>::default().into()
    }
}

impl Default for PostCreatePayloadAndUpdatePayload {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            subject: Self::default_subject(),
            body: Self::default_body(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct PostPopulatedGetResult {
    pub id: PostId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub subject: String,
    pub body: String,
    pub comment_ids: Vec<CommentId>,
    pub reactions: Vec<Reaction>,
    pub poll: Option<Poll>,
    pub images: Vec<PostImage>,
}

impl PostPopulatedGetResult {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> PostId {
        <PostId as Default>::default().into()
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

    pub fn default_subject() -> String {
        <String as Default>::default().into()
    }

    pub fn default_body() -> String {
        <String as Default>::default().into()
    }

    pub fn default_comment_ids() -> Vec<CommentId> {
        <Vec<CommentId> as Default>::default().into()
    }

    pub fn default_reactions() -> Vec<Reaction> {
        <Vec<Reaction> as Default>::default().into()
    }

    pub fn default_poll() -> Option<Poll> {
        None
    }

    pub fn default_images() -> Vec<PostImage> {
        <Vec<PostImage> as Default>::default().into()
    }
}

sqlx_json_decode!(PostPopulatedGetResult);

impl Default for PostPopulatedGetResult {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            subject: Self::default_subject(),
            body: Self::default_body(),
            comment_ids: Self::default_comment_ids(),
            reactions: Self::default_reactions(),
            poll: Self::default_poll(),
            images: Self::default_images(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, schemars::JsonSchema, sqlx::FromRow, Serialize)]
pub struct PostPopulatedListResult {
    pub id: PostId,
    pub organization_id: crate::models::organization::OrganizationId,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub subject: String,
    pub body: String,
    pub comment_ids: Vec<CommentId>,
    pub poll_id: Option<PollId>,
}

impl PostPopulatedListResult {
    // The <T as Default> syntax here is weird but lets us generate from the template without needing to
    // detect whether to add the extra :: in cases like DateTime::<Utc>::default

    pub fn default_id() -> PostId {
        <PostId as Default>::default().into()
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

    pub fn default_subject() -> String {
        <String as Default>::default().into()
    }

    pub fn default_body() -> String {
        <String as Default>::default().into()
    }

    pub fn default_comment_ids() -> Vec<CommentId> {
        <Vec<CommentId> as Default>::default().into()
    }

    pub fn default_poll_id() -> Option<PollId> {
        None
    }
}

sqlx_json_decode!(PostPopulatedListResult);

impl Default for PostPopulatedListResult {
    fn default() -> Self {
        Self {
            id: Self::default_id(),
            organization_id: Self::default_organization_id(),
            updated_at: Self::default_updated_at(),
            created_at: Self::default_created_at(),
            subject: Self::default_subject(),
            body: Self::default_body(),
            comment_ids: Self::default_comment_ids(),
            poll_id: Self::default_poll_id(),
        }
    }
}
