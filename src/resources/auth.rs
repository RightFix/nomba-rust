//! Authentication resource for Nomba API
//!
//! Provides methods for managing access tokens.

use crate::error::Result;
use crate::http_client::BlockingNombaClient;
use crate::http_client::NombaClient;
use crate::models::{RefreshTokenResponse, RevokeTokenResponse};
use serde_json::json;

/// Synchronous authentication client.
#[derive(Clone)]
pub struct Auth {
    client: BlockingNombaClient,
}

impl Auth {
    /// Creates a new `Auth` resource.
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    /// Revokes an access token.
    ///
    /// Use this endpoint to invalidate an access token, for example when a user logs out.
    ///
    /// # Arguments
    /// * `access_token` - The JWT access token to revoke
    ///
    /// # Returns
    /// A [`RevokeTokenResponse`] indicating success or failure.
    ///
    /// # Example
    /// ```no_run
    /// use nomba_rs::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let revoked = nomba.auth.revoke_access_token("access_token_to_revoke".to_string())?;
    /// println!("Token revoked: {}", revoked.description);
    /// # Ok::<(), nomba_rs::NombaError>(())
    /// ```
    pub fn revoke_access_token(
        &self,
        access_token: impl Into<String>,
    ) -> Result<RevokeTokenResponse> {
        let body = json!({
            "clientId": self.client.inner.config.client_id,
            "access_token": access_token.into(),
        });
        let response = self.client.post("/v1/auth/token/revoke", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Refreshes an expired access token.
    ///
    /// Exchange a `refresh_token` (from token issue) for a new
    /// `access_token`. See `POST /v1/auth/token/refresh` in the
    /// [Nomba docs](https://developer.nomba.com/nomba-api-reference/authenticate/refresh-an-expired-token).
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token to exchange
    pub fn refresh_access_token(
        &self,
        refresh_token: impl Into<String>,
    ) -> Result<RefreshTokenResponse> {
        let body = json!({
            "grant_type": "refresh_token",
            "refresh_token": refresh_token.into(),
        });
        let response = self.client.post("/v1/auth/token/refresh", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

/// Asynchronous authentication client.
#[derive(Clone)]
pub struct AsyncAuth {
    client: NombaClient,
}

impl AsyncAuth {
    /// Creates a new `AsyncAuth` resource.
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Revokes an access token.
    ///
    /// Use this endpoint to invalidate an access token, for example when a user logs out.
    ///
    /// # Arguments
    /// * `access_token` - The JWT access token to revoke
    ///
    /// # Returns
    /// A [`RevokeTokenResponse`] indicating success or failure.
    ///
    /// # Example
    /// ```no_run
    /// use nomba_rs::AsyncNomba;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> nomba_rs::Result<()> {
    /// let nomba = AsyncNomba::new("client_id", "client_secret", "account_id").await?;
    /// let revoked = nomba.auth.revoke_access_token("access_token_to_revoke".to_string()).await?;
    /// println!("Token revoked: {}", revoked.description);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn revoke_access_token(
        &self,
        access_token: impl Into<String>,
    ) -> Result<RevokeTokenResponse> {
        let body = json!({
            "clientId": self.client.config.client_id,
            "access_token": access_token.into(),
        });
        let response = self
            .client
            .post("/v1/auth/token/revoke", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Refreshes an expired access token.
    ///
    /// See [`Auth::refresh_access_token`] for details.
    pub async fn refresh_access_token(
        &self,
        refresh_token: impl Into<String>,
    ) -> Result<RefreshTokenResponse> {
        let body = json!({
            "grant_type": "refresh_token",
            "refresh_token": refresh_token.into(),
        });
        let response = self
            .client
            .post("/v1/auth/token/refresh", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
