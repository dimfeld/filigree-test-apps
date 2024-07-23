use async_trait::async_trait;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing};
use axum_jsonschema::Json;
use error_stack::{Report, ResultExt};
use filigree::extract::FormOrJson;
use schemars::JsonSchema;
use serde::Serialize;
use sqlx::{PgConnection, PgExecutor};

use crate::{
    auth::Authed,
    models::{
        organization::OrganizationId,
        user::{User, UserCreatePayload, UserId},
    },
    server::ServerState,
    Error,
};

/// The current user and other information to return to the client.
#[derive(Serialize, Debug, JsonSchema)]
pub struct SelfUser {
    roles: Vec<crate::models::role::RoleId>,
    permissions: Vec<String>,
}

async fn get_current_user_endpoint(
    State(state): State<ServerState>,
    authed: Authed,
) -> Result<impl IntoResponse, Error> {
    // CUSTOM-AUTH Potentially add additional user information here

    let user = SelfUser {
        roles: authed.roles.clone(),
        permissions: authed.permissions.clone(),
    };

    Ok(Json(user))
}

pub fn create_routes() -> axum::Router<ServerState> {
    axum::Router::new().route("/self", routing::get(get_current_user_endpoint))
}

#[cfg(test)]
mod test {
    use crate::tests::{start_app, BootstrappedData};

    #[sqlx::test]
    async fn get_current_user(db: sqlx::PgPool) {
        let (app, BootstrappedData { admin_user, .. }) = start_app(db).await;

        let user_info: serde_json::Value = admin_user
            .client
            .get("self")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
    }
}
