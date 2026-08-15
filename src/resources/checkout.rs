use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Checkout {
    client: BlockingNombaClient,
}

impl Checkout {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn create_order(
        &self,
        order_reference: impl Into<String>,
        amount: impl Into<String>,
        currency: impl Into<String>,
        customer_email: impl Into<String>,
        customer_name: impl Into<String>,
        redirect_url: impl Into<String>,
        description: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Result<CreateCheckoutOrderResponse> {
        let mut body = json!({
            "orderReference": order_reference.into(),
            "amount": amount.into(),
            "currency": currency.into(),
            "customerEmail": customer_email.into(),
            "customerName": customer_name.into(),
            "redirectUrl": redirect_url.into(),
        });

        if let Some(description) = description {
            body["description"] = json!(description);
        }
        if let Some(metadata) = metadata {
            body["metadata"] = metadata;
        }

        let response = self.client.post("/v1/checkout/order", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_transaction(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionResponse> {
        let params = vec![
            ("idType", "ORDER_REFERENCE".to_string()),
            ("id", order_reference.into()),
        ];
        let response = self.client.get("/v1/checkout/transaction", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn refund_transaction(
        &self,
        order_reference: impl Into<String>,
        amount: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<RefundCheckoutResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "amount": amount.into(),
            "reason": reason.into(),
        });
        let response = self.client.post("/v1/checkout/refund", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_order_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutOrderDetailsResponse> {
        let path = format!("/v1/checkout/order/{}", order_reference.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn submit_card_details(
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

    pub fn submit_otp(
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

    pub fn resend_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/resend-otp", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let path = format!("/v1/checkout/order/{}", order_reference.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_flash_account(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchFlashAccountResponse> {
        let path = format!("/v1/checkout/get-checkout-kta/{}", order_reference.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn request_user_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<RequestUserOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/user-card/auth", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn submit_user_otp(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<SubmitUserOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "otp": otp.into(),
        });
        let response = self.client.post("/v1/checkout/user-card/saved-card/auth", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_user_saved_cards(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchUserSavedCardsResponse> {
        let path = format!("/v1/checkout/user-card/{}", order_reference.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn cancel_transaction(
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

    pub fn cancel_order(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<CancelCheckoutOrderResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/order/cancel", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncCheckout {
    client: NombaClient,
}

impl AsyncCheckout {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn create_order(
        &self,
        order_reference: impl Into<String>,
        amount: impl Into<String>,
        currency: impl Into<String>,
        customer_email: impl Into<String>,
        customer_name: impl Into<String>,
        redirect_url: impl Into<String>,
        description: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Result<CreateCheckoutOrderResponse> {
        let mut body = json!({
            "orderReference": order_reference.into(),
            "amount": amount.into(),
            "currency": currency.into(),
            "customerEmail": customer_email.into(),
            "customerName": customer_name.into(),
            "redirectUrl": redirect_url.into(),
        });

        if let Some(description) = description {
            body["description"] = json!(description);
        }
        if let Some(metadata) = metadata {
            body["metadata"] = metadata;
        }

        let response = self.client.post("/v1/checkout/order", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_transaction(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionResponse> {
        let params = vec![
            ("idType", "ORDER_REFERENCE".to_string()),
            ("id", order_reference.into()),
        ];
        let response = self.client.get("/v1/checkout/transaction", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn refund_transaction(
        &self,
        order_reference: impl Into<String>,
        amount: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<RefundCheckoutResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "amount": amount.into(),
            "reason": reason.into(),
        });
        let response = self.client.post("/v1/checkout/refund", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_order_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutOrderDetailsResponse> {
        let path = format!("/v1/checkout/order/{}", order_reference.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn submit_card_details(
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

    pub async fn submit_otp(
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

    pub async fn resend_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/resend-otp", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let path = format!("/v1/checkout/order/{}", order_reference.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_flash_account(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchFlashAccountResponse> {
        let path = format!("/v1/checkout/get-checkout-kta/{}", order_reference.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn request_user_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<RequestUserOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/user-card/auth", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn submit_user_otp(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<SubmitUserOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "otp": otp.into(),
        });
        let response = self.client.post("/v1/checkout/user-card/saved-card/auth", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_user_saved_cards(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchUserSavedCardsResponse> {
        let path = format!("/v1/checkout/user-card/{}", order_reference.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn cancel_transaction(
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

    pub async fn cancel_order(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<CancelCheckoutOrderResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/order/cancel", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}
