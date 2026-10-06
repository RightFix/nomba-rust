use crate::error::Result;
use crate::http_client::BlockingNombaClient;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct GlobalCollections {
    client: BlockingNombaClient,
}

impl GlobalCollections {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    /// Triggers a mobile money collection request from a customer.
    ///
    /// Calls `POST /v1/global-collection/inflow/initiate`. Use
    /// [`Self::fetch_drc_inflow_providers`] for valid `topup_vendor` values
    /// and store the returned `idempotencyKey` for safe retries.
    ///
    /// # Arguments
    /// * `phone_number` - Customer phone number (e.g., "0980802xxx")
    /// * `callback_url` - Webhook URL for collection status updates
    /// * `amount` - Amount to collect
    /// * `currency` - Currency code (e.g., "CDF")
    /// * `topup_vendor` - Mobile money network provider (e.g., "AIRTEL", "MPESA")
    /// * `idempotency_key` - Optional client-generated retry key; the server
    ///   generates one when omitted
    pub fn initiate_mobile_money_inflow(
        &self,
        phone_number: impl Into<String>,
        callback_url: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
        topup_vendor: impl Into<String>,
        idempotency_key: Option<String>,
    ) -> Result<InitiateMobileMoneyInflowResponse> {
        let mut body = json!({
            "phoneNumber": phone_number.into(),
            "callbackUrl": callback_url.into(),
            "amount": amount,
            "currency": currency.into(),
            "topupVendor": topup_vendor.into(),
        });
        if let Some(idempotency_key) = idempotency_key {
            body["idempotencyKey"] = json!(idempotency_key);
        }
        let response = self
            .client
            .post("/v1/global-collection/inflow/initiate", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_collection_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchCollectionTransactionResponse> {
        let path = format!(
            "/v1/global-collection/transactions/{}",
            transaction_id.into()
        );
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Lists mobile money providers supported for DRC inflow.
    ///
    /// Calls `GET /v1/global-collection/drc/inflow/providers`. Use the
    /// returned codes as `topup_vendor` in
    /// [`Self::initiate_mobile_money_inflow`].
    pub fn fetch_drc_inflow_providers(&self) -> Result<FetchDrcInflowProvidersResponse> {
        let response = self
            .client
            .get("/v1/global-collection/drc/inflow/providers", None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sandbox version of [`Self::fetch_drc_inflow_providers`].
    ///
    /// Calls `GET /v1/sandbox/global-collection/drc/inflow/providers` and
    /// returns canned provider data for testing.
    pub fn fetch_drc_inflow_providers_sandbox(&self) -> Result<FetchDrcInflowProvidersResponse> {
        let response = self
            .client
            .get("/v1/sandbox/global-collection/drc/inflow/providers", None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncGlobalCollections {
    client: NombaClient,
}

impl AsyncGlobalCollections {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Triggers a mobile money collection request from a customer.
    ///
    /// See [`GlobalCollections::initiate_mobile_money_inflow`] for details.
    pub async fn initiate_mobile_money_inflow(
        &self,
        phone_number: impl Into<String>,
        callback_url: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
        topup_vendor: impl Into<String>,
        idempotency_key: Option<String>,
    ) -> Result<InitiateMobileMoneyInflowResponse> {
        let mut body = json!({
            "phoneNumber": phone_number.into(),
            "callbackUrl": callback_url.into(),
            "amount": amount,
            "currency": currency.into(),
            "topupVendor": topup_vendor.into(),
        });
        if let Some(idempotency_key) = idempotency_key {
            body["idempotencyKey"] = json!(idempotency_key);
        }
        let response = self
            .client
            .post("/v1/global-collection/inflow/initiate", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_collection_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchCollectionTransactionResponse> {
        let path = format!(
            "/v1/global-collection/transactions/{}",
            transaction_id.into()
        );
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Lists mobile money providers supported for DRC inflow.
    ///
    /// See [`GlobalCollections::fetch_drc_inflow_providers`] for details.
    pub async fn fetch_drc_inflow_providers(&self) -> Result<FetchDrcInflowProvidersResponse> {
        let response = self
            .client
            .get("/v1/global-collection/drc/inflow/providers", None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Sandbox version of [`GlobalCollections::fetch_drc_inflow_providers`].
    pub async fn fetch_drc_inflow_providers_sandbox(
        &self,
    ) -> Result<FetchDrcInflowProvidersResponse> {
        let response = self
            .client
            .get("/v1/sandbox/global-collection/drc/inflow/providers", None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
