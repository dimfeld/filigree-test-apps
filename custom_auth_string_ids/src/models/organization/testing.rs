#![allow(unused_imports, unused_variables, dead_code)]
use super::{OrganizationCreatePayload, OrganizationId, OrganizationUpdatePayload};

/// Generate a OrganizationCreatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_create_payload(i: usize) -> OrganizationCreatePayload {
    OrganizationCreatePayload {
        id: None,
        name: format!("Test object {i}"),
        owner: (i > 1).then(|| format!("Test object {i}")),
        default_role: (i > 1).then(|| format!("Test object {i}")),
    }
}

/// Generate a OrganizationUpdatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_update_payload(i: usize) -> OrganizationUpdatePayload {
    OrganizationUpdatePayload {
        id: None,
        name: format!("Test object {i}"),
        owner: Some(format!("Test object {i}")),
        default_role: Some(format!("Test object {i}")),
    }
}
