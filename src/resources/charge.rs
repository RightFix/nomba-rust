use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Charge {
    client: BlockingNombaClient,
}

impl Charge {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn charge_with_tokenized_card(
        &self,
        amount: impl Into<String>,
        currency: impl Into<String>,
        tokenized_card_id: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_email: impl Into<String>,
        customer_name: impl Into<String>,
        description: Option<String>,
    ) -> Result<ChargeWithTokenizedCardResponse> {
        let mut body = json!({
            "amount": amount.into(),
            "currency": currency.into(),
            "tokenizedCardId": tokenized_card_id.into(),
            "merchantTxRef": merchant_tx_ref.into(),
            "customerEmail": customer_email.into(),
            "customerName": customer_name.into(),
        });
        if let Some(description) = description {
            body["description"] = json!(description);
        }
        let response = self.client.post("/v1/charge/tokenized", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn list_tokenized_cards(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ListTokenizedCardsResponse> {
        let mut params = Vec::new();
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        let response = self.client.get("/v1/charge/tokenized", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn update_tokenized_card(
        &self,
        tokenized_card_id: impl Into<String>,
        status: impl Into<String>,
    ) -> Result<UpdateTokenizedCardResponse> {
        let path = format!("/v1/charge/tokenized/{}", tokenized_card_id.into());
        let body = json!({ "status": status.into() });
        let response = self.client.put(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn delete_tokenized_card(
        &self,
        tokenized_card_id: impl Into<String>,
    ) -> Result<DeleteTokenizedCardResponse> {
        let path = format!("/v1/charge/tokenized/{}", tokenized_card_id.into());
        let response = self.client.delete(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_bank_codes(&self) -> Result<FetchBankCodesResponse> {
        let response = self.client.get("/v1/charge/banks", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn bank_account_lookup(
        &self,
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
    ) -> Result<BankAccountLookupResponse> {
        let path = format!("/v1/charge/banks/lookup");
        let body = json!({
            "accountNumber": account_number.into(),
            "bankCode": bank_code.into(),
        });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn submit_customer_card_details(
        &self,
        order_reference: impl Into<String>,
        card_details: impl Into<String>,
        key: impl Into<String>,
        save_card: Option<bool>,
        device_information: Option<serde_json::Value>,
    ) -> Result<SubmitCardDetailsResponse> {
        let path = format!("/v1/charge/orders/{}/card", order_reference.into());
        let mut body = json!({
            "cardDetails": card_details.into(),
            "key": key.into(),
        });
        if let Some(save_card) = save_card {
            body["saveCard"] = json!(save_card);
        }
        if let Some(device_information) = device_information {
            body["deviceInformation"] = device_information;
        }
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn submit_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Result<SubmitOtpResponse> {
        let path = format!("/v1/charge/orders/{}/otp", order_reference.into());
        let body = json!({
            "otp": otp.into(),
            "transactionId": transaction_id.into(),
        });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn resend_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let path = format!("/v1/charge/orders/{}/otp/resend", order_reference.into());
        let response = self.client.post(&path, &json!({}), None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_checkout_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let path = format!("/v1/charge/orders/{}/details", order_reference.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn cancel_checkout_transaction(
        &self,
        order_reference: impl Into<String>,
        transaction_id: impl Into<String>,
        force: Option<bool>,
    ) -> Result<CancelCheckoutTransactionResponse> {
        let path = format!("/v1/charge/orders/{}/transactions/{}/cancel", order_reference.into(), transaction_id.into());
        let mut body = json!({});
        if let Some(force) = force {
            body["force"] = json!(force);
        }
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncCharge {
    client: NombaClient,
}

impl AsyncCharge {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn charge_with_tokenized_card(
        &self,
        amount: impl Into<String>,
        currency: impl Into<String>,
        tokenized_card_id: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_email: impl Into<String>,
        customer_name: impl Into<String>,
        description: Option<String>,
    ) -> Result<ChargeWithTokenizedCardResponse> {
        let mut body = json!({
            "amount": amount.into(),
            "currency": currency.into(),
            "tokenizedCardId": tokenized_card_id.into(),
            "merchantTxRef": merchant_tx_ref.into(),
            "customerEmail": customer_email.into(),
            "customerName": customer_name.into(),
        });
        if let Some(description) = description {
            body["description"] = json!(description);
        }
        let response = self.client.post("/v1/charge/tokenized", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn list_tokenized_cards(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ListTokenizedCardsResponse> {
        let mut params = Vec::new();
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        let response = self.client.get("/v1/charge/tokenized", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update_tokenized_card(
        &self,
        tokenized_card_id: impl Into<String>,
        status: impl Into<String>,
    ) -> Result<UpdateTokenizedCardResponse> {
        let path = format!("/v1/charge/tokenized/{}", tokenized_card_id.into());
        let body = json!({ "status": status.into() });
        let response = self.client.put(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete_tokenized_card(
        &self,
        tokenized_card_id: impl Into<String>,
    ) -> Result<DeleteTokenizedCardResponse> {
        let path = format!("/v1/charge/tokenized/{}", tokenized_card_id.into());
        let response = self.client.delete(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_bank_codes(&self) -> Result<FetchBankCodesResponse> {
        let response = self.client.get("/v1/charge/banks", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn bank_account_lookup(
        &self,
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
    ) -> Result<BankAccountLookupResponse> {
        let path = format!("/v1/charge/banks/lookup");
        let body = json!({
            "accountNumber": account_number.into(),
            "bankCode": bank_code.into(),
        });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn submit_customer_card_details(
        &self,
        order_reference: impl Into<String>,
        card_details: impl Into<String>,
        key: impl Into<String>,
        save_card: Option<bool>,
        device_information: Option<serde_json::Value>,
    ) -> Result<SubmitCardDetailsResponse> {
        let path = format!("/v1/charge/orders/{}/card", order_reference.into());
        let mut body = json!({
            "cardDetails": card_details.into(),
            "key": key.into(),
        });
        if let Some(save_card) = save_card {
            body["saveCard"] = json!(save_card);
        }
        if let Some(device_information) = device_information {
            body["deviceInformation"] = device_information;
        }
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn submit_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Result<SubmitOtpResponse> {
        let path = format!("/v1/charge/orders/{}/otp", order_reference.into());
        let body = json!({
            "otp": otp.into(),
            "transactionId": transaction_id.into(),
        });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn resend_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let path = format!("/v1/charge/orders/{}/otp/resend", order_reference.into());
        let response = self.client.post(&path, &json!({}), None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_checkout_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let path = format!("/v1/charge/orders/{}/details", order_reference.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn cancel_checkout_transaction(
        &self,
        order_reference: impl Into<String>,
        transaction_id: impl Into<String>,
        force: Option<bool>,
    ) -> Result<CancelCheckoutTransactionResponse> {
        let path = format!("/v1/charge/orders/{}/transactions/{}/cancel", order_reference.into(), transaction_id.into());
        let mut body = json!({});
        if let Some(force) = force {
            body["force"] = json!(force);
        }
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}