use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Terminals {
    client: BlockingNombaClient,
}

impl Terminals {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn assign_to_account(
        &self,
        account_id: impl Into<String>,
        terminal_id: impl Into<String>,
    ) -> Result<AssignTerminalResponse> {
        let path = format!("/v1/terminals/assign/{}", account_id.into());
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn assign_to_parent_account(
        &self,
        terminal_id: impl Into<String>,
    ) -> Result<AssignTerminalResponse> {
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post("/v1/terminals/assign", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn unassign_from_account(
        &self,
        account_id: impl Into<String>,
        terminal_id: impl Into<String>,
    ) -> Result<UnassignTerminalResponse> {
        let path = format!("/v1/terminals/unassign/{}", account_id.into());
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn unassign_from_parent_account(
        &self,
        terminal_id: impl Into<String>,
    ) -> Result<UnassignTerminalResponse> {
        let path = "/v1/terminals/unassign".to_string();
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn send_payment_request(
        &self,
        terminal_id: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
        order_reference: impl Into<String>,
        customer_name: Option<String>,
        customer_phone: Option<String>,
        customer_email: Option<String>,
    ) -> Result<SendPaymentRequestResponse> {
        let path = format!("/v1/terminals/payment-request/{}", terminal_id.into());
        let mut body = json!({
            "amount": amount,
            "currency": currency.into(),
            "orderReference": order_reference.into(),
        });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncTerminals {
    client: NombaClient,
}

impl AsyncTerminals {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn assign_to_account(
        &self,
        account_id: impl Into<String>,
        terminal_id: impl Into<String>,
    ) -> Result<AssignTerminalResponse> {
        let path = format!("/v1/terminals/assign/{}", account_id.into());
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn assign_to_parent_account(
        &self,
        terminal_id: impl Into<String>,
    ) -> Result<AssignTerminalResponse> {
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post("/v1/terminals/assign", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn unassign_from_account(
        &self,
        account_id: impl Into<String>,
        terminal_id: impl Into<String>,
    ) -> Result<UnassignTerminalResponse> {
        let path = format!("/v1/terminals/unassign/{}", account_id.into());
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn unassign_from_parent_account(
        &self,
        terminal_id: impl Into<String>,
    ) -> Result<UnassignTerminalResponse> {
        let path = "/v1/terminals/unassign".to_string();
        let body = json!({ "terminalId": terminal_id.into() });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn send_payment_request(
        &self,
        terminal_id: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
        order_reference: impl Into<String>,
        customer_name: Option<String>,
        customer_phone: Option<String>,
        customer_email: Option<String>,
    ) -> Result<SendPaymentRequestResponse> {
        let path = format!("/v1/terminals/payment-request/{}", terminal_id.into());
        let mut body = json!({
            "amount": amount,
            "currency": currency.into(),
            "orderReference": order_reference.into(),
        });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}