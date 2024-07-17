#![allow(unused_imports, unused_variables, dead_code)]
use super::{PostCreatePayload, PostId, PostUpdatePayload};
use crate::models::{
    comment::{
        Comment, CommentCreatePayload, CommentCreateResult, CommentId, CommentUpdatePayload,
    },
    poll::{Poll, PollCreatePayload, PollCreateResult, PollId, PollUpdatePayload},
    post_image::{
        PostImage, PostImageCreatePayload, PostImageCreateResult, PostImageId,
        PostImageUpdatePayload,
    },
    reaction::{
        Reaction, ReactionCreatePayload, ReactionCreateResult, ReactionId, ReactionUpdatePayload,
    },
};

/// Generate a PostCreatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_create_payload(i: usize) -> PostCreatePayload {
    PostCreatePayload {
        id: None,

        subject: format!("Test object {i}"),
        body: format!("Test object {i}"),
    }
}

/// Generate a PostUpdatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_update_payload(i: usize) -> PostUpdatePayload {
    PostUpdatePayload {
        id: None,

        subject: format!("Test object {i}"),
        body: format!("Test object {i}"),
    }
}
