//! CableTV resource for Nomba API
//!
//! Provides methods for cable TV subscription and plan lookup.

use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

/// Synchronous CableTV client.
#[derive(Clone)]
pub struct CableTv {
    client: BlockingNombaClient,
}

impl CableTv {
    /// Creates a new `CableTv` resource.
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    /// Looks up a cable TV smart card number.
    ///
    /// Validates the smart card number and returns the customer name and bouquet details.
    ///
    /// # Arguments
    /// * `provider` - Cable TV provider (e.g., "dstv", "gotv", "startimes")
    /// * `smart_card_number` - The smart card number to verify
    ///
    /// # Returns
    /// A [`CableTvLookupResponse`] with customer details.
    pub fn lookup(&self, provider: impl Into<String>, smart_card_number: impl Into<String>) -> Result<CableTvLookupResponse> {
        let path = format!("/v1/cabletv/lookup/{}/{}", provider.into(), smart_card_number.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Subscribes to a cable TV plan via parent account.
    ///
    /// # Arguments
    /// * `provider` - Cable TV provider (e.g., "dstv", "gotv", "startimes")
    /// * `smart_card_number` - Customer's smart card number
    /// * `plan` - Plan code (e.g., "COMPACT", "PADI")
    /// * `amount` - Subscription amount in kobo
    /// * `merchant_tx_ref` - Unique transaction reference
    /// * `phone_number` - Optional customer phone number
    ///
    /// # Returns
    /// A [`CableTvSubscriptionResponse`] with transaction details.
    pub fn subscribe_parent(
        &self,
        provider: impl Into<String>,
        smart_card_number: impl Into<String>,
        plan: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
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
        let response = self.client.post("/v1/cabletv/parent", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Subscribes to a cable TV plan via sub-account.
    ///
    /// # Arguments
    /// * `account_id` - Sub-account ID
    /// * `provider` - Cable TV provider (e.g., "dstv", "gotv", "startimes")
    /// * `smart_card_number` - Customer's smart card number
    /// * `plan` - Plan code (e.g., "COMPACT", "PADI")
    /// * `amount` - Subscription amount in kobo
    /// * `merchant_tx_ref` - Unique transaction reference
    /// * `phone_number` - Optional customer phone number
    ///
    /// # Returns
    /// A [`CableTvSubscriptionResponse`] with transaction details.
    pub fn subscribe_account(
        &self,
        account_id: impl Into<String>,
        provider: impl Into<String>,
        smart_card_number: impl Into<String>,
        plan: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
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
        let path = format!("/v1/cabletv/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches available cable TV plans for a provider.
    ///
    /// Returns a list of plans with their subscription types and amounts.
    ///
    /// # Arguments
    /// * `cable_tv_type` - Cable TV type: "dstv", "gotv", "startimes", or "ShowMax"
    ///
    /// # Returns
    /// A [`FetchCableTvPlansResponse`] containing the list of plans.
    ///
    /// # Example
    /// ```no_run
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let plans = nomba.cabletv.fetch_plans("dstv")?;
    /// for plan in plans.data {
    ///     println!("Plan: {} - ₦{}", plan.subscription_type, plan.amount);
    /// }
    /// # Ok::<(), nomba::NombaError>(())
    /// ```
    pub fn fetch_plans(&self, cable_tv_type: impl Into<String>) -> Result<FetchCableTvPlansResponse> {
        let mut params = Vec::new();
        params.push(("cableTvType", cable_tv_type.into()));
        let response = self.client.get("/v1/bill/cableTvProduct", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }
}

/// Asynchronous CableTV client.
#[derive(Clone)]
pub struct AsyncCableTv {
    client: NombaClient,
}

impl AsyncCableTv {
    /// Creates a new `AsyncCableTv` resource.
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Looks up a cable TV smart card number.
    pub async fn lookup(&self, provider: impl Into<String>, smart_card_number: impl Into<String>) -> Result<CableTvLookupResponse> {
        let path = format!("/v1/cabletv/lookup/{}/{}", provider.into(), smart_card_number.into());
        let response = self.client.get(&path, None).await?;
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
        let response = self.client.post("/v1/cabletv/parent", &body, None).await?;
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
        let path = format!("/v1/cabletv/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches available cable TV plans for a provider.
    ///
    /// See [`CableTv::fetch_plans`] for full parameter documentation.
    pub async fn fetch_plans(&self, cable_tv_type: impl Into<String>) -> Result<FetchCableTvPlansResponse> {
        let mut params = Vec::new();
        params.push(("cableTvType", cable_tv_type.into()));
        let response = self.client.get("/v1/bill/cableTvProduct", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }
}