#![allow(unused_imports, dead_code)]
use filigree::auth::ObjectPermission;
use sea_orm::prelude::*;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use sqlx_transparent_json_decode::sqlx_json_decode;

use super::{entity::ActiveModel, PostId};
use crate::models::{
    comment::{Comment, CommentCreatePayload, CommentId, CommentUpdatePayload},
    organization::OrganizationId,
    poll::{Poll, PollCreatePayload, PollId, PollUpdatePayload},
    post_image::{PostImage, PostImageCreatePayload, PostImageId, PostImageUpdatePayload},
    reaction::{Reaction, ReactionCreatePayload, ReactionId, ReactionUpdatePayload},
};

pub type Model = super::entity::Model;

pub type PostUserView = Model;

pub type PostOwnerView = Model;

pub type PostListResult = Model;

pub type PostCreateResult = Model;

pub type Post = Model;

impl Model {
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

sqlx_json_decode!(Model);

impl Default for Model {
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
