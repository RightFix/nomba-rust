use crate::http_client::BlockingNombaClient;
use crate::error::Result;
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

    pub fn initiate_mobile_money_inflow(
        &self,
        amount: f64,
        currency: impl Into<String>,
        phone_number: impl Into<String>,
        provider: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_name: Option<String>,
        customer_email: Option<String>,
        callback_url: Option<String>,
    ) -> Result<InitiateMobileMoneyInflowResponse> {
        let mut body = json!({
            "amount": amount,
            "currency": currency.into(),
            "phoneNumber": phone_number.into(),
            "topupVendor": provider.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(customer_name) = customer_name {
            body["customerName"] = json!(customer_name);
        }
        if let Some(customer_email) = customer_email {
            body["customerEmail"] = json!(customer_email);
        }
        if let Some(callback_url) = callback_url {
            body["callbackUrl"] = json!(callback_url);
        }
        let response = self.client.post("/v1/global-collection/inflow/initiate", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_collection_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchCollectionTransactionResponse> {
        let path = format!("/v1/global-collection/transactions/{}", transaction_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_drc_inflow_providers(&self, sandbox: Option<bool>) -> Result<FetchDrcInflowProvidersResponse> {
        let mut params = Vec::new();
        if let Some(sandbox) = sandbox {
            params.push(("sandbox", sandbox.to_string()));
        }
        let response = self.client.get("/v1/global-collection/drc/inflow/providers", Some(params))?;
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

    pub async fn initiate_mobile_money_inflow(
        &self,
        amount: f64,
        currency: impl Into<String>,
        phone_number: impl Into<String>,
        provider: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_name: Option<String>,
        customer_email: Option<String>,
        callback_url: Option<String>,
    ) -> Result<InitiateMobileMoneyInflowResponse> {
        let mut body = json!({
            "amount": amount,
            "currency": currency.into(),
            "phoneNumber": phone_number.into(),
            "topupVendor": provider.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(customer_name) = customer_name {
            body["customerName"] = json!(customer_name);
        }
        if let Some(customer_email) = customer_email {
            body["customerEmail"] = json!(customer_email);
        }
        if let Some(callback_url) = callback_url {
            body["callbackUrl"] = json!(callback_url);
        }
        let response = self.client.post("/v1/global-collection/inflow/initiate", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_collection_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchCollectionTransactionResponse> {
        let path = format!("/v1/global-collection/transactions/{}", transaction_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_drc_inflow_providers(&self, sandbox: Option<bool>) -> Result<FetchDrcInflowProvidersResponse> {
        let mut params = Vec::new();
        if let Some(sandbox) = sandbox {
            params.push(("sandbox", sandbox.to_string()));
        }
        let response = self.client.get("/v1/global-collection/drc/inflow/providers", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }
}
