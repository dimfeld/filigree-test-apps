pub mod queries;
#[cfg(test)]
pub mod testing;
pub mod types;

pub use types::*;

pub const READ_PERMISSION: &str = "PostTag::read";
pub const WRITE_PERMISSION: &str = "PostTag::write";
pub const OWNER_PERMISSION: &str = "PostTag::owner";

pub const CREATE_PERMISSION: &str = "PostTag::owner";
