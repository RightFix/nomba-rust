use crate::error::Result;
use crate::http_client::BlockingNombaClient;
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
        let response = self
            .client
            .post("/v1/checkout/checkout-card-detail", &body, None)?;
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
        let response = self
            .client
            .post("/v1/checkout/checkout-card-otp", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn resend_otp(&self, order_reference: impl Into<String>) -> Result<ResendOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self.client.post("/v1/checkout/resend-otp", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self
            .client
            .post("/v1/checkout/confirm-transaction-receipt", &body, None)?;
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

    /// Requests an OTP to authenticate a user before saving their card.
    ///
    /// Calls `POST /v1/checkout/user-card/auth`. Invoke after a successful
    /// payment when the user asked to save their card, then complete with
    /// [`Self::submit_user_otp`].
    pub fn request_user_otp(
        &self,
        order_reference: impl Into<String>,
        phone_number: impl Into<String>,
    ) -> Result<RequestUserOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "phoneNumber": phone_number.into(),
        });
        let response = self
            .client
            .post("/v1/checkout/user-card/auth", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Requests an OTP to validate a user who already has saved cards.
    ///
    /// Calls `POST /v1/checkout/user-card/saved-card/auth`. Use when
    /// `fetch_order_details` indicates saved cards exist, then retrieve them
    /// with [`Self::fetch_user_saved_cards`] using the OTP.
    pub fn request_saved_cards_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<RequestUserOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self
            .client
            .post("/v1/checkout/user-card/saved-card/auth", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Submits the user OTP sent to the user's phone.
    ///
    /// Calls `POST /v1/checkout/user-card`. This saves the user's card for
    /// later use.
    pub fn submit_user_otp(
        &self,
        order_reference: impl Into<String>,
        phone_number: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<SubmitUserOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "phoneNumber": phone_number.into(),
            "otp": otp.into(),
        });
        let response = self.client.post("/v1/checkout/user-card", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches a user's saved cards.
    ///
    /// Calls `GET /v1/checkout/user-card/{orderReference}?otp={otp}`.
    /// Obtain `otp` via [`Self::request_saved_cards_otp`] first.
    pub fn fetch_user_saved_cards(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<FetchUserSavedCardsResponse> {
        let path = format!("/v1/checkout/user-card/{}", order_reference.into());
        let params = vec![("otp", otp.into())];
        let response = self.client.get(&path, Some(params))?;
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
        let response = self
            .client
            .post("/v1/checkout/transaction/cancel", &body, None)?;
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

    /// Charges a customer using tokenized card data.
    ///
    /// Calls `POST /v1/checkout/tokenized-card-payment`.
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
        let response = self
            .client
            .post("/v1/checkout/tokenized-card-payment", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Lists all tokenized cards for the merchant.
    ///
    /// Calls `GET /v1/checkout/tokenized-card-data`.
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
        let response = self
            .client
            .get("/v1/checkout/tokenized-card-data", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates a tokenized card's email mapping.
    ///
    /// Calls `POST /v1/checkout/tokenized-card-data`.
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
        let response = self
            .client
            .post("/v1/checkout/tokenized-card-data", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a tokenized card.
    ///
    /// Calls `DELETE /v1/checkout/tokenized-card-data`.
    pub fn delete_tokenized_card(
        &self,
        token_key: impl Into<String>,
    ) -> Result<DeleteTokenizedCardResponse> {
        let params = vec![("tokenKey", token_key.into())];
        let response = self
            .client
            .delete("/v1/checkout/tokenized-card-data", Some(params))?;
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
        let response = self
            .client
            .get("/v1/checkout/transaction", Some(params))
            .await?;
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
        let response = self
            .client
            .post("/v1/checkout/checkout-card-detail", &body, None)
            .await?;
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
        let response = self
            .client
            .post("/v1/checkout/checkout-card-otp", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn resend_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<ResendOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self
            .client
            .post("/v1/checkout/resend-otp", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_transaction_details(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<FetchCheckoutTransactionDetailsResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self
            .client
            .post("/v1/checkout/confirm-transaction-receipt", &body, None)
            .await?;
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

    /// Requests an OTP to authenticate a user before saving their card.
    ///
    /// See [`Checkout::request_user_otp`] for details.
    pub async fn request_user_otp(
        &self,
        order_reference: impl Into<String>,
        phone_number: impl Into<String>,
    ) -> Result<RequestUserOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "phoneNumber": phone_number.into(),
        });
        let response = self
            .client
            .post("/v1/checkout/user-card/auth", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Requests an OTP to validate a user who already has saved cards.
    ///
    /// See [`Checkout::request_saved_cards_otp`] for details.
    pub async fn request_saved_cards_otp(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<RequestUserOtpResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self
            .client
            .post("/v1/checkout/user-card/saved-card/auth", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Submits the user OTP sent to the user's phone.
    ///
    /// See [`Checkout::submit_user_otp`] for details.
    pub async fn submit_user_otp(
        &self,
        order_reference: impl Into<String>,
        phone_number: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<SubmitUserOtpResponse> {
        let body = json!({
            "orderReference": order_reference.into(),
            "phoneNumber": phone_number.into(),
            "otp": otp.into(),
        });
        let response = self
            .client
            .post("/v1/checkout/user-card", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches a user's saved cards.
    ///
    /// See [`Checkout::fetch_user_saved_cards`] for details.
    pub async fn fetch_user_saved_cards(
        &self,
        order_reference: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<FetchUserSavedCardsResponse> {
        let path = format!("/v1/checkout/user-card/{}", order_reference.into());
        let params = vec![("otp", otp.into())];
        let response = self.client.get(&path, Some(params)).await?;
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
        let response = self
            .client
            .post("/v1/checkout/transaction/cancel", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn cancel_order(
        &self,
        order_reference: impl Into<String>,
    ) -> Result<CancelCheckoutOrderResponse> {
        let body = json!({ "orderReference": order_reference.into() });
        let response = self
            .client
            .post("/v1/checkout/order/cancel", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Charges a customer using tokenized card data.
    ///
    /// See [`Checkout::charge_with_tokenized_card`] for details.
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
        let response = self
            .client
            .post("/v1/checkout/tokenized-card-payment", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Lists all tokenized cards for the merchant.
    ///
    /// See [`Checkout::list_tokenized_cards`] for details.
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
        let response = self
            .client
            .get("/v1/checkout/tokenized-card-data", Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Updates a tokenized card's email mapping.
    ///
    /// See [`Checkout::update_tokenized_card`] for details.
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
        let response = self
            .client
            .post("/v1/checkout/tokenized-card-data", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Deletes a tokenized card.
    ///
    /// See [`Checkout::delete_tokenized_card`] for details.
    pub async fn delete_tokenized_card(
        &self,
        token_key: impl Into<String>,
    ) -> Result<DeleteTokenizedCardResponse> {
        let params = vec![("tokenKey", token_key.into())];
        let response = self
            .client
            .delete("/v1/checkout/tokenized-card-data", Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
