use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Electricity {
    client: BlockingNombaClient,
}

impl Electricity {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn fetch_providers(&self) -> Result<FetchElectricityProvidersResponse> {
        let response = self.client.get("/v1/electricity/providers", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn customer_lookup(&self, provider: impl Into<String>, meter_number: impl Into<String>) -> Result<ElectricityCustomerLookupResponse> {
        let path = format!("/v1/electricity/lookup/{}/{}", provider.into(), meter_number.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn vend_parent(
        &self,
        provider: impl Into<String>,
        meter_number: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
        meter_type: Option<String>,
    ) -> Result<VendElectricityResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "meterNumber": meter_number.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        if let Some(meter_type) = meter_type {
            body["meterType"] = json!(meter_type);
        }
        let response = self.client.post("/v1/electricity/parent", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn vend_account(
        &self,
        account_id: impl Into<String>,
        provider: impl Into<String>,
        meter_number: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
        meter_type: Option<String>,
    ) -> Result<VendElectricityResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "meterNumber": meter_number.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        if let Some(meter_type) = meter_type {
            body["meterType"] = json!(meter_type);
        }
        let path = format!("/v1/electricity/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncElectricity {
    client: NombaClient,
}

impl AsyncElectricity {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn fetch_providers(&self) -> Result<FetchElectricityProvidersResponse> {
        let response = self.client.get("/v1/electricity/providers", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn customer_lookup(&self, provider: impl Into<String>, meter_number: impl Into<String>) -> Result<ElectricityCustomerLookupResponse> {
        let path = format!("/v1/electricity/lookup/{}/{}", provider.into(), meter_number.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn vend_parent(
        &self,
        provider: impl Into<String>,
        meter_number: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
        meter_type: Option<String>,
    ) -> Result<VendElectricityResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "meterNumber": meter_number.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        if let Some(meter_type) = meter_type {
            body["meterType"] = json!(meter_type);
        }
        let response = self.client.post("/v1/electricity/parent", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn vend_account(
        &self,
        account_id: impl Into<String>,
        provider: impl Into<String>,
        meter_number: impl Into<String>,
        amount: f64,
        merchant_tx_ref: impl Into<String>,
        phone_number: Option<String>,
        meter_type: Option<String>,
    ) -> Result<VendElectricityResponse> {
        let mut body = json!({
            "provider": provider.into(),
            "meterNumber": meter_number.into(),
            "amount": amount,
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(phone_number) = phone_number {
            body["phoneNumber"] = json!(phone_number);
        }
        if let Some(meter_type) = meter_type {
            body["meterType"] = json!(meter_type);
        }
        let path = format!("/v1/electricity/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}