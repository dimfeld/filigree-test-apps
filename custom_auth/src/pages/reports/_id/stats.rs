#![allow(unused_imports)]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use axum_extra::extract::{Form, Query};
use filigree::extract::ValidatedForm;
use maud::{html, Markup};
use schemars::JsonSchema;

use crate::{
    auth::{has_any_permission, Authed},
    pages::{auth::WebAuthed, error::HtmlError, layout::root_layout_page},
    server::ServerState,
    Error,
};

async fn stats_page(
    State(state): State<ServerState>,
    auth: WebAuthed,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, HtmlError> {
    let body = html! {};

    Ok(root_layout_page(Some(&auth), "title", body))
}

pub fn create_routes() -> axum::Router<ServerState> {
    axum::Router::new().route(
        "/reports/:id/stats",
        routing::get(stats_page).route_layer(has_any_permission(vec!["Report:read", "org_admin"])),
    )
}
