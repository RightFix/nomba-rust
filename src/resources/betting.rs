use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Betting {
    client: NombaClient,
}

impl Betting {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn fetch_providers(&self) -> Result<FetchBettingProvidersResponse> {
        let response = self.client.get("/v1/bill/betting/providers", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn customer_lookup(
        &self,
        provider: impl Into<String>,
        customer_id: impl Into<String>,
    ) -> Result<BettingCustomerLookupResponse> {
        // Docs: `GET /v1/bill/betting/lookup` takes `providerId` (the code
        // from the providers list, e.g. `bet9ja`) + `customerId`. The old
        // `provider` name is rejected with a 400.
        let params = vec![
            ("providerId", provider.into()),
            ("customerId", customer_id.into()),
        ];
        let response = self
            .client
            .get("/v1/bill/betting/lookup", Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn vend_parent(
        &self,
        provider: impl Into<String>,
        customer_id: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
    ) -> Result<VendBettingResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "customerId": customer_id.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        let response = self.client.post("/v2/bill/betting", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn vend_account(
        &self,
        account_id: impl Into<String>,
        provider: impl Into<String>,
        customer_id: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
    ) -> Result<VendBettingResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "customerId": customer_id.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        let path = format!("/v2/bill/betting/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}
