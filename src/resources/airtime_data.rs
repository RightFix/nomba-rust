//! Airtime & Data Vending resource for Nomba API
//!
//! Provides methods for airtime purchase and data bundle vending.

use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

/// Synchronous Airtime & Data client.
#[derive(Clone)]
pub struct AirtimeData {
    client: BlockingNombaClient,
}

impl AirtimeData {
    /// Creates a new `AirtimeData` resource.
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    /// Purchases airtime via parent account.
    ///
    /// # Arguments
    /// * `amount` - Airtime amount in kobo
    /// * `phone_number` - Recipient phone number
    /// * `network` - Network provider (e.g., "MTN", "GLO", "AIRTEL", "9MOBILE")
    /// * `merchant_tx_ref` - Unique transaction reference
    /// * `sender_name` - Optional sender name
    ///
    /// # Returns
    /// An [`AirtimePurchaseResponse`] with transaction details.
    pub fn purchase_airtime_parent(
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
        let response = self.client.post("/v1/bill/topup", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Purchases airtime via sub-account.
    ///
    /// # Arguments
    /// * `account_id` - Sub-account ID
    /// * `amount` - Airtime amount in kobo
    /// * `phone_number` - Recipient phone number
    /// * `network` - Network provider (e.g., "MTN", "GLO", "AIRTEL", "9MOBILE")
    /// * `merchant_tx_ref` - Unique transaction reference
    /// * `sender_name` - Optional sender name
    ///
    /// # Returns
    /// An [`AirtimePurchaseResponse`] with transaction details.
    pub fn purchase_airtime_account(
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
        let path = format!("/v1/bill/topup/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Vends data bundles via parent account.
    ///
    /// Uses `product_id` (not `amount`) to identify the exact data plan.
    /// Call [`Self::fetch_data_plans`] first to get available product IDs.
    ///
    /// # Arguments
    /// * `product_id` - Product ID from `fetch_data_plans` (e.g., "mtn47")
    /// * `phone_number` - Recipient phone number
    /// * `network` - Network provider (e.g., "MTN", "GLO", "AIRTEL", "9MOBILE")
    /// * `merchant_tx_ref` - Unique transaction reference
    /// * `sender_name` - Optional sender name
    ///
    /// # Returns
    /// A [`DataVendingResponse`] with transaction details.
    ///
    /// # Example
    /// ```no_run
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let plans = nomba.airtime_data.fetch_data_plans("MTN".to_string())?;
    /// let vend = nomba.airtime_data.vend_data_parent(
    ///     "mtn47".to_string(), "08055441122".to_string(), "MTN".to_string(), "txn-ref-123".to_string(), Some("John Doe".to_string()),
    /// )?;
    /// println!("Data vended: {}", vend.data.amount);
    /// # Ok::<(), nomba::NombaError>(())
    /// ```
    pub fn vend_data_parent(
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
        let response = self.client.post("/v1/bill/data", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Vends data bundles via sub-account.
    ///
    /// Uses `product_id` (not `amount`) to identify the exact data plan.
    /// Call [`Self::fetch_data_plans`] first to get available product IDs.
    ///
    /// # Arguments
    /// * `account_id` - Sub-account ID
    /// * `product_id` - Product ID from `fetch_data_plans` (e.g., "mtn47")
    /// * `phone_number` - Recipient phone number
    /// * `network` - Network provider (e.g., "MTN", "GLO", "AIRTEL", "9MOBILE")
    /// * `merchant_tx_ref` - Unique transaction reference
    /// * `sender_name` - Optional sender name
    ///
    /// # Returns
    /// A [`DataVendingResponse`] with transaction details.
    pub fn vend_data_account(
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
        let path = format!("/v1/bill/data/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches available data plans for a network.
    ///
    /// Returns a list of plans with their product IDs and amounts.
    /// Use the `productId` from the response when calling `vend_data_parent` or `vend_data_account`.
    ///
    /// # Arguments
    /// * `network` - Network provider (e.g., "MTN", "GLO", "AIRTEL", "9MOBILE")
    ///
    /// # Returns
    /// A [`FetchDataPlansResponse`] containing the list of plans.
    pub fn fetch_data_plans(&self, network: impl Into<String>) -> Result<FetchDataPlansResponse> {
        let path = format!("/v1/bill/data-plan/{}", network.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

/// Asynchronous Airtime & Data client.
#[derive(Clone)]
pub struct AsyncAirtimeData {
    client: NombaClient,
}

impl AsyncAirtimeData {
    /// Creates a new `AsyncAirtimeData` resource.
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
        let response = self.client.post("/v1/bill/topup", &body, None).await?;
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
        let path = format!("/v1/bill/topup/{}", account_id.into());
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
        let response = self.client.post("/v1/bill/data", &body, None).await?;
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
        let path = format!("/v1/bill/data/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches available data plans for a network.
    ///
    /// See [`AirtimeData::fetch_data_plans`] for full parameter documentation.
    pub async fn fetch_data_plans(&self, network: impl Into<String>) -> Result<FetchDataPlansResponse> {
        let path = format!("/v1/bill/data-plan/{}", network.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}