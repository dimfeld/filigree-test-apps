#![allow(unused_imports, unused_variables, dead_code)]
use super::{ReportTagCreatePayload, ReportTagUpdatePayload};
use crate::models::{report::ReportId, tag::TagId};

/// Generate a ReportTagCreatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_create_payload(i: usize) -> ReportTagCreatePayload {
    ReportTagCreatePayload {
        report_id: None,

        tag_id: None,
    }
}

/// Generate a ReportTagUpdatePayload for testing.
/// Parameter `i` controls the value of some of the fields, just to make sure that the objects
/// don't all look identical.
pub fn make_update_payload(i: usize) -> ReportTagUpdatePayload {
    ReportTagUpdatePayload {
        report_id: None,

        tag_id: None,
    }
}
