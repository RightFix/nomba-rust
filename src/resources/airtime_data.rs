//! Airtime & Data Vending resource for Nomba API
//!
//! Provides methods for airtime purchase and data bundle vending.

use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

/// Airtime & Data client.
#[derive(Clone)]
pub struct AirtimeData {
    client: NombaClient,
}

impl AirtimeData {
    /// Creates a new `AirtimeData` resource.
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Purchases airtime via parent account.
    pub async fn purchase_airtime_parent(
        &self,
        amount: f64,
        phone_number: impl Into<String>,
        network: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        sender_name: Option<String>,
    ) -> Result<AirtimePurchaseResponse> {
        let mut body = json!({
            "amount": amount,
            "phoneNumber": phone_number.into(),
            "network": network.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(sender_name) = sender_name {
            body["senderName"] = json!(sender_name);
        }
        let response = self.client.post("/v2/bill/topup", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Purchases airtime via sub-account.
    pub async fn purchase_airtime_account(
        &self,
        account_id: impl Into<String>,
        amount: f64,
        phone_number: impl Into<String>,
        network: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        sender_name: Option<String>,
    ) -> Result<AirtimePurchaseResponse> {
        let mut body = json!({
            "amount": amount,
            "phoneNumber": phone_number.into(),
            "network": network.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(sender_name) = sender_name {
            body["senderName"] = json!(sender_name);
        }
        let path = format!("/v2/bill/topup/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Vends data bundles via parent account.
    ///
    /// See [`AirtimeData::vend_data_parent`] for full parameter documentation.
    pub async fn vend_data_parent(
        &self,
        product_id: impl Into<String>,
        phone_number: impl Into<String>,
        network: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        sender_name: Option<String>,
    ) -> Result<DataVendingResponse> {
        let mut body = json!({
            "productId": product_id.into(),
            "phoneNumber": phone_number.into(),
            "network": network.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(sender_name) = sender_name {
            body["senderName"] = json!(sender_name);
        }
        let response = self.client.post("/v2/bill/data", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Vends data bundles via sub-account.
    ///
    /// See [`AirtimeData::vend_data_account`] for full parameter documentation.
    pub async fn vend_data_account(
        &self,
        account_id: impl Into<String>,
        product_id: impl Into<String>,
        phone_number: impl Into<String>,
        network: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        sender_name: Option<String>,
    ) -> Result<DataVendingResponse> {
        let mut body = json!({
            "productId": product_id.into(),
            "phoneNumber": phone_number.into(),
            "network": network.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(sender_name) = sender_name {
            body["senderName"] = json!(sender_name);
        }
        let path = format!("/v2/bill/data/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches available data plans for a network.
    ///
    /// See [`AirtimeData::fetch_data_plans`] for full parameter documentation.
    pub async fn fetch_data_plans(
        &self,
        network: impl Into<String>,
    ) -> Result<FetchDataPlansResponse> {
        let path = format!("/v1/bill/data-plan/{}", network.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}
