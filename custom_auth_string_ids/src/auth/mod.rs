use std::borrow::Cow;

use async_trait::async_trait;
use axum::routing::{self, Router};
use error_stack::{Report, ResultExt};
use filigree::auth::{
    AuthError, ExpiryStyle, OrganizationId, PermissionChecker, RoleId, SessionKey,
    UserFromRequestPartsValue, UserId,
};
use sqlx::{query_file_as, PgPool};
use uuid::Uuid;

use crate::server::ServerState;

pub mod permissions;
#[cfg(test)]
mod tests;

pub type Authed = filigree::auth::Authed<AuthInfo>;

#[derive(Debug, sqlx::FromRow)]
pub struct AuthInfo {
    /// The user id of this user
    pub user_id: UserId,
    /// The organization id of this user
    pub organization_id: OrganizationId,
    /// If this user is enabled.
    pub active: bool,
    /// The user's roles
    pub roles: Vec<RoleId>,
    /// The permission for the user and all their roles.
    pub permissions: Vec<String>,
    /// True if this user was authenticated as an anonymous fallback.
    pub anonymous: bool,
}

impl AuthInfo {
    pub fn actor_ids(&self) -> Vec<String> {
        self.roles
            .iter()
            .cloned()
            .chain(std::iter::once(self.user_id.clone()))
            .collect::<Vec<_>>()
    }
}

impl filigree::auth::AuthInfo for AuthInfo {
    fn check_valid(&self) -> Result<(), AuthError> {
        if !self.active {
            Err(AuthError::Disabled)
        } else {
            Ok(())
        }
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }

    fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }
}

pub struct AuthQueries {
    db: PgPool,
    session_expiry_style: filigree::auth::ExpiryStyle,
}

impl AuthQueries {
    pub fn new(db: PgPool, session_expiry_style: filigree::auth::ExpiryStyle) -> Self {
        Self {
            db,
            session_expiry_style,
        }
    }
}

#[async_trait]
impl filigree::auth::AuthQueries for AuthQueries {
    type AuthInfo = AuthInfo;

    async fn get_user_by_api_key(
        &self,
        api_key: Uuid,
        hash: Vec<u8>,
    ) -> Result<Option<AuthInfo>, Report<AuthError>> {
        Ok(None)
    }

    async fn get_user_by_session_id(
        &self,
        session_key: &SessionKey,
    ) -> Result<Option<AuthInfo>, Report<AuthError>> {
        Ok(None)
    }

    async fn anonymous_user(&self, user_id: UserId) -> Result<Option<AuthInfo>, Report<AuthError>> {
        Ok(None)
    }

    async fn get_user_from_request_parts(
        &self,
        parts: &http::request::Parts,
    ) -> Result<UserFromRequestPartsValue<AuthInfo>, Report<AuthError>> {
        // CUSTOM-AUTH Fill in your custom logic here
        Ok(UserFromRequestPartsValue::NotImplemented)
    }
}

pub fn has_permission(
    permission: impl Into<Cow<'static, str>>,
) -> filigree::auth::HasPermissionLayer<AuthInfo, impl PermissionChecker<AuthInfo>> {
    filigree::auth::has_permission(permission.into())
}

pub fn has_any_permission(
    permissions: Vec<impl Into<Cow<'static, str>>>,
) -> filigree::auth::HasPermissionLayer<AuthInfo, impl PermissionChecker<AuthInfo>> {
    filigree::auth::has_any_permission(permissions)
}

pub fn has_all_permissions(
    permissions: Vec<impl Into<Cow<'static, str>>>,
) -> filigree::auth::HasPermissionLayer<AuthInfo, impl PermissionChecker<AuthInfo>> {
    filigree::auth::has_all_permissions(permissions)
}

/// Disallow anonymous users
pub fn not_anonymous() -> filigree::auth::NotAnonymousLayer<AuthInfo> {
    filigree::auth::not_anonymous()
}

pub fn has_auth_predicate<F>(
    message: impl Into<Cow<'static, str>>,
    f: F,
) -> filigree::auth::HasPredicateLayer<AuthInfo, F>
where
    F: Fn(&AuthInfo) -> bool + Clone,
{
    filigree::auth::has_auth_predicate(message.into(), f)
}

pub fn create_routes() -> Router<ServerState> {
    Router::new()
}
