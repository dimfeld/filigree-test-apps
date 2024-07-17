#![allow(unused_imports, unused_variables, dead_code)]
use super::{TagCreatePayload, TagId, TagUpdatePayload};

/// Generate a TagCreatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_create_payload(i: usize) -> TagCreatePayload {
    TagCreatePayload {
        id: None,

        name: format!("Test object {i}"),
        color: format!("Test object {i}"),
    }
}

/// Generate a TagUpdatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_update_payload(i: usize) -> TagUpdatePayload {
    TagUpdatePayload {
        id: None,

        name: format!("Test object {i}"),
        color: format!("Test object {i}"),
    }
}
