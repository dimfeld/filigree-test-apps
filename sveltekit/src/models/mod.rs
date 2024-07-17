pub mod comment;
pub mod organization;
pub mod poll;
pub mod post;
pub mod post_image;
pub mod post_tag;
pub mod reaction;
pub mod report;
pub mod report_section;
pub mod report_tag;
pub mod role;
pub mod tag;
pub mod user;

use axum::Router;

use crate::server::ServerState;

pub fn create_routes() -> Router<ServerState> {
    Router::new()
        .merge(post::endpoints::create_routes())
        .merge(report::endpoints::create_routes())
        .merge(role::endpoints::create_routes())
        .merge(tag::endpoints::create_routes())
        .merge(user::endpoints::create_routes())
}
