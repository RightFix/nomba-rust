//! Direct Debits resource for Nomba API
//!
//! Provides methods for creating and managing direct debit mandates.

use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

/// Direct Debits client.
#[derive(Clone)]
pub struct DirectDebits {
    client: NombaClient,
}

impl DirectDebits {
    /// Creates a new `DirectDebits` resource.
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Gets mandates with optional filters.
    pub async fn get_mandates(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
        status: Option<String>,
        customer_account_number: Option<String>,
        customer_name: Option<String>,
    ) -> Result<GetMandatesResponse> {
        let mut params = Vec::new();
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            params.push(("pageSize", page_size.to_string()));
        }
        if let Some(status) = status {
            params.push(("status", status));
        }
        if let Some(customer_account_number) = customer_account_number {
            params.push(("customerAccountNumber", customer_account_number));
        }
        if let Some(customer_name) = customer_name {
            params.push(("customerName", customer_name));
        }
        let response = self
            .client
            .get("/v1/direct-debits/mandates", Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates a mandate status.
    pub async fn update_mandate_status(
        &self,
        mandate_id: impl Into<String>,
        mandate_status: impl Into<String>,
    ) -> Result<UpdateMandateStatusResponse> {
        let body =
            json!({ "mandateId": mandate_id.into(), "mandateStatus": mandate_status.into() });
        let response = self
            .client
            .put("/v1/direct-debits/update-status", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Debits a customer's account using an active mandate.
    pub async fn debit_mandate(
        &self,
        mandate_id: impl Into<String>,
        amount: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
    ) -> Result<DebitMandateResponse> {
        let body = json!({
            "mandateId": mandate_id.into(),
            "amount": amount.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        let response = self
            .client
            .post("/v1/direct-debits/debit-mandate", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets the status of a specific mandate.
    pub async fn get_mandate_status(
        &self,
        mandate_id: impl Into<String>,
    ) -> Result<GetMandateStatusResponse> {
        let path = format!("/v1/direct-debits/status?mandateId={}", mandate_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Gets full details of a mandate by ID.
    pub async fn get_mandate_by_id(
        &self,
        mandate_id: impl Into<String>,
    ) -> Result<GetMandateByIdResponse> {
        let path = format!("/v1/direct-debits/{}", mandate_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Creates a new direct debit mandate.
    ///
    /// See [`DirectDebits::create_mandate`] for full parameter documentation.
    pub async fn create_mandate(
        &self,
        customer_account_number: impl Into<String>,
        customer_account_name: impl Into<String>,
        bank_code: impl Into<String>,
        amount: f64,
        customer_name: impl Into<String>,
        customer_address: impl Into<String>,
        customer_email: impl Into<String>,
        customer_phone_number: impl Into<String>,
        merchant_reference: impl Into<String>,
        frequency: impl Into<String>,
        start_date: Vec<i32>,
        end_date: Vec<i32>,
        narration: Option<String>,
        start_immediately: Option<bool>,
    ) -> Result<CreateMandateResponse> {
        let mut body = json!({
            "customerAccountNumber": customer_account_number.into(),
            "customerAccountName": customer_account_name.into(),
            "bankCode": bank_code.into(),
            "amount": amount,
            "customerName": customer_name.into(),
            "customerAddress": customer_address.into(),
            "customerEmail": customer_email.into(),
            "customerPhoneNumber": customer_phone_number.into(),
            "merchantReference": merchant_reference.into(),
            "frequency": frequency.into(),
            "startDate": start_date,
            "endDate": end_date,
        });
        if let Some(narration) = narration {
            body["narration"] = json!(narration);
        }
        if let Some(start_immediately) = start_immediately {
            body["startImmediately"] = json!(start_immediately);
        }
        let response = self.client.post("/v1/direct-debits", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}
