pub mod endpoints;
pub mod queries;
#[cfg(test)]
pub mod testing;
pub mod types;

pub use types::*;

pub const READ_PERMISSION: &str = "Tag::read";
pub const WRITE_PERMISSION: &str = "Tag::write";
pub const OWNER_PERMISSION: &str = "Tag::owner";

pub const CREATE_PERMISSION: &str = "Tag::owner";

filigree::make_object_id!(TagId, tag);
