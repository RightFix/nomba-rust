use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct GlobalCollections {
    client: NombaClient,
}

impl GlobalCollections {
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
