//! Authentication resource for Nomba API
//!
//! Provides methods for managing access tokens.

use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::RevokeTokenResponse;
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
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let revoked = nomba.auth.revoke_access_token("access_token_to_revoke".to_string())?;
    /// println!("Token revoked: {}", revoked.description);
    /// # Ok::<(), nomba::NombaError>(())
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
    /// use nomba::AsyncNomba;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> nomba::Result<()> {
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
        let response = self.client.post("/v1/auth/token/revoke", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}