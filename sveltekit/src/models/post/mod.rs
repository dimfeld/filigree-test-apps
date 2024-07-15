pub mod endpoints;
pub(super) mod entity;
pub mod queries;
#[cfg(test)]
pub mod testing;
pub mod types;

/// The base entity allows executing any query but does not automatically attach auth checks. You
/// should often use Entity instead, which is a wrapper that automatically checks for authorization on
/// any query.
pub use entity::Entity as BaseEntity;
pub use entity::{ActiveModel, AuthedEntity as Entity, Column, Model, PrimaryKey, Relation};
pub use types::*;

pub const NAME: &str = "Post";
pub const READ_PERMISSION: &str = "Post::read";
pub const WRITE_PERMISSION: &str = "Post::write";
pub const OWNER_PERMISSION: &str = "Post::owner";

pub const CREATE_PERMISSION: &str = OWNER_PERMISSION;

filigree::make_object_id!(PostId, pst);
