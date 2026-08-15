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
        let response = self.client.post("/v1/checkout/tokenized-card-payment", &body, None)?;
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
        let response = self.client.get("/v1/checkout/tokenized-card-data", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn update_tokenized_card(
        &self,
        token_key: impl Into<String>,
        current_email_address: impl Into<String>,
        new_email_address: impl Into<String>,
    ) -> Result<UpdateTokenizedCardResponse> {
        let body = json!({
            "tokenKey": token_key.into(),
            "currentEmailAddress": current_email_address.into(),
            "newEmailAddress": new_email_address.into(),
        });
        let response = self.client.post("/v1/checkout/tokenized-card-data", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn delete_tokenized_card(
        &self,
        token_key: impl Into<String>,
    ) -> Result<DeleteTokenizedCardResponse> {
        let params = vec![("tokenKey", token_key.into())];
        let response = self.client.delete("/v1/checkout/tokenized-card-data", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_bank_codes(&self) -> Result<FetchBankCodesResponse> {
        let response = self.client.get("/v1/transfers/banks", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn bank_account_lookup(
        &self,
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
    ) -> Result<BankAccountLookupResponse> {
        let body = json!({
            "accountNumber": account_number.into(),
            "bankCode": bank_code.into(),
        });
        let response = self.client.post("/v1/transfers/bank/lookup", &body, None)?;
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
        let mut body = json!({
            "orderReference": order_reference.into(),
            "cardDetails": card_details.into(),
            "key": key.into(),
        });
        if let Some(save_card) = save_card {
            body["saveCard"] = json!(save_card);
        }
        if let Some(device_information) = device_information {
            body["deviceInformation"] = device_information;
        }
        let response = self.client.post("/v1/checkout/checkout-card-detail", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn submit_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Result<SubmitOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "otp": otp.into(),
            "transactionId": transaction_id.into(),
        });
        let response = self.client.post("/v1/checkout/checkout-card-otp", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn resend_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/resend-otp", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_checkout_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/confirm-transaction-receipt", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn cancel_checkout_transaction(
        &self,
        transaction_id: impl Into<String>,
        force: Option<bool>,
    ) -> Result<CancelCheckoutTransactionResponse> {
        let body = json!({
            "transactionId": transaction_id.into(),
            "forceCancel": force.unwrap_or(false),
        });
        let response = self.client.post("/v1/checkout/transaction/cancel", &body, None)?;
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
        let response = self.client.post("/v1/checkout/tokenized-card-payment", &body, None).await?;
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
        let response = self.client.get("/v1/checkout/tokenized-card-data", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update_tokenized_card(
        &self,
        token_key: impl Into<String>,
        current_email_address: impl Into<String>,
        new_email_address: impl Into<String>,
    ) -> Result<UpdateTokenizedCardResponse> {
        let body = json!({
            "tokenKey": token_key.into(),
            "currentEmailAddress": current_email_address.into(),
            "newEmailAddress": new_email_address.into(),
        });
        let response = self.client.post("/v1/checkout/tokenized-card-data", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete_tokenized_card(
        &self,
        token_key: impl Into<String>,
    ) -> Result<DeleteTokenizedCardResponse> {
        let params = vec![("tokenKey", token_key.into())];
        let response = self.client.delete("/v1/checkout/tokenized-card-data", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_bank_codes(&self) -> Result<FetchBankCodesResponse> {
        let response = self.client.get("/v1/transfers/banks", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn bank_account_lookup(
        &self,
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
    ) -> Result<BankAccountLookupResponse> {
        let body = json!({
            "accountNumber": account_number.into(),
            "bankCode": bank_code.into(),
        });
        let response = self.client.post("/v1/transfers/bank/lookup", &body, None).await?;
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
        let mut body = json!({
            "orderReference": order_reference.into(),
            "cardDetails": card_details.into(),
            "key": key.into(),
        });
        if let Some(save_card) = save_card {
            body["saveCard"] = json!(save_card);
        }
        if let Some(device_information) = device_information {
            body["deviceInformation"] = device_information;
        }
        let response = self.client.post("/v1/checkout/checkout-card-detail", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn submit_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Result<SubmitOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "otp": otp.into(),
            "transactionId": transaction_id.into(),
        });
        let response = self.client.post("/v1/checkout/checkout-card-otp", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn resend_customer_payment_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/resend-otp", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_checkout_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/confirm-transaction-receipt", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn cancel_checkout_transaction(
        &self,
        transaction_id: impl Into<String>,
        force: Option<bool>,
    ) -> Result<CancelCheckoutTransactionResponse> {
        let body = json!({
            "transactionId": transaction_id.into(),
            "forceCancel": force.unwrap_or(false),
        });
        let response = self.client.post("/v1/checkout/transaction/cancel", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}
