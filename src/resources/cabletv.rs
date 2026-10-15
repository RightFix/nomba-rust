//! CableTV resource for Nomba API
//!
//! Provides methods for cable TV subscription and plan lookup.

use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

/// CableTV client.
#[derive(Clone)]
pub struct CableTv {
    client: NombaClient,
}

impl CableTv {
    /// Creates a new `CableTv` resource.
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Looks up a cable TV smart card number.
    pub async fn lookup(
        &self,
        provider: impl Into<String>,
        smart_card_number: impl Into<String>,
    ) -> Result<CableTvLookupResponse> {
        let params = vec![
            ("provider", provider.into()),
            ("smartCardNumber", smart_card_number.into()),
        ];
        let response = self
            .client
            .get("/v1/bill/cabletv/lookup", Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Subscribes to a cable TV plan via parent account.
    pub async fn subscribe_parent(
        &self,
        provider: impl Into<String>,
        smart_card_number: impl Into<String>,
        plan: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
        payer_name: Option<String>,
    ) -> Result<CableTvSubscriptionResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "smartCardNumber": smart_card_number.into(),
            "plan": plan.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        if let Some(payer_name) = payer_name {
            body["payerName"] = json!(payer_name);
        }
        let response = self.client.post("/v2/bill/cabletv", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Subscribes to a cable TV plan via sub-account.
    pub async fn subscribe_account(
        &self,
        account_id: impl Into<String>,
        provider: impl Into<String>,
        smart_card_number: impl Into<String>,
        plan: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
        payer_name: Option<String>,
    ) -> Result<CableTvSubscriptionResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "smartCardNumber": smart_card_number.into(),
            "plan": plan.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        if let Some(payer_name) = payer_name {
            body["payerName"] = json!(payer_name);
        }
        let path = format!("/v2/bill/cabletv/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches available cable TV plans for a provider.
    ///
    /// See [`CableTv::fetch_plans`] for full parameter documentation.
    pub async fn fetch_plans(
        &self,
        cable_tv_type: impl Into<String>,
    ) -> Result<FetchCableTvPlansResponse> {
        let params = vec![("cableTvType", cable_tv_type.into())];
        let response = self
            .client
            .get("/v1/bill/cableTvProduct", Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
