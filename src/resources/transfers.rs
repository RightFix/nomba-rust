use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Transfers {
    client: BlockingNombaClient,
}

impl Transfers {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn bank_transfer_from_parent(
        &self,
        amount: impl Into<String>,
        destination_account_number: impl Into<String>,
        destination_bank_code: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_name: Option<String>,
    ) -> Result<PerformBankTransferResponse> {
        let mut body = json!({
            "amount": amount.into(),
            "destinationAccountNumber": destination_account_number.into(),
            "destinationBankCode": destination_bank_code.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(customer_name) = customer_name {
            body["customerName"] = json!(customer_name);
        }
        let response = self.client.post("/v1/transfers/bank", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn bank_transfer_from_account(
        &self,
        account_id: impl Into<String>,
        amount: impl Into<String>,
        destination_account_number: impl Into<String>,
        destination_bank_code: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_name: Option<String>,
    ) -> Result<PerformBankTransferResponse> {
        let mut body = json!({
            "amount": amount.into(),
            "destinationAccountNumber": destination_account_number.into(),
            "destinationBankCode": destination_bank_code.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(customer_name) = customer_name {
            body["customerName"] = json!(customer_name);
        }
        let path = format!("/v1/transfers/bank/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn wallet_transfer_from_parent(
        &self,
        amount: f64,
        destination_wallet_id: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
    ) -> Result<PerformWalletTransferResponse> {
        let body = json!({
            "amount": amount,
            "destinationWalletId": destination_wallet_id.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        let response = self.client.post("/v1/transfers/wallet", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn wallet_transfer_from_account(
        &self,
        account_id: impl Into<String>,
        amount: f64,
        destination_wallet_id: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
    ) -> Result<PerformWalletTransferResponse> {
        let body = json!({
            "amount": amount,
            "destinationWalletId": destination_wallet_id.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        let path = format!("/v1/transfers/wallet/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn authorize_transfer(
        &self,
        transaction_id: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<AuthorizeTransferResponse> {
        let path = format!("/v1/transfers/{}/authorize", transaction_id.into());
        let body = json!({ "otp": otp.into() });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn authorize_exchange(
        &self,
        transaction_id: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<AuthorizeExchangeResponse> {
        let path = format!("/v1/transfers/{}/exchange/authorize", transaction_id.into());
        let body = json!({ "otp": otp.into() });
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn convert_money(
        &self,
        from_amount: f64,
        from_currency: impl Into<String>,
        to_currency: impl Into<String>,
    ) -> Result<ConvertMoneyResponse> {
        let body = json!({
            "fromAmount": from_amount,
            "fromCurrency": from_currency.into(),
            "toCurrency": to_currency.into(),
        });
        let response = self.client.post("/v1/transfers/convert", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_exchange_rates(&self) -> Result<FetchExchangeRatesResponse> {
        let response = self.client.get("/v1/transfers/rates", None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncTransfers {
    client: NombaClient,
}

impl AsyncTransfers {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn bank_transfer_from_parent(
        &self,
        amount: impl Into<String>,
        destination_account_number: impl Into<String>,
        destination_bank_code: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_name: Option<String>,
    ) -> Result<PerformBankTransferResponse> {
        let mut body = json!({
            "amount": amount.into(),
            "destinationAccountNumber": destination_account_number.into(),
            "destinationBankCode": destination_bank_code.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(customer_name) = customer_name {
            body["customerName"] = json!(customer_name);
        }
        let response = self.client.post("/v1/transfers/bank", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn bank_transfer_from_account(
        &self,
        account_id: impl Into<String>,
        amount: impl Into<String>,
        destination_account_number: impl Into<String>,
        destination_bank_code: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
        customer_name: Option<String>,
    ) -> Result<PerformBankTransferResponse> {
        let mut body = json!({
            "amount": amount.into(),
            "destinationAccountNumber": destination_account_number.into(),
            "destinationBankCode": destination_bank_code.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        if let Some(customer_name) = customer_name {
            body["customerName"] = json!(customer_name);
        }
        let path = format!("/v1/transfers/bank/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn wallet_transfer_from_parent(
        &self,
        amount: f64,
        destination_wallet_id: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
    ) -> Result<PerformWalletTransferResponse> {
        let body = json!({
            "amount": amount,
            "destinationWalletId": destination_wallet_id.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        let response = self.client.post("/v1/transfers/wallet", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn wallet_transfer_from_account(
        &self,
        account_id: impl Into<String>,
        amount: f64,
        destination_wallet_id: impl Into<String>,
        narration: impl Into<String>,
        merchant_tx_ref: impl Into<String>,
    ) -> Result<PerformWalletTransferResponse> {
        let body = json!({
            "amount": amount,
            "destinationWalletId": destination_wallet_id.into(),
            "narration": narration.into(),
            "merchantTxRef": merchant_tx_ref.into(),
        });
        let path = format!("/v1/transfers/wallet/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn authorize_transfer(
        &self,
        transaction_id: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<AuthorizeTransferResponse> {
        let path = format!("/v1/transfers/{}/authorize", transaction_id.into());
        let body = json!({ "otp": otp.into() });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn authorize_exchange(
        &self,
        transaction_id: impl Into<String>,
        otp: impl Into<String>,
    ) -> Result<AuthorizeExchangeResponse> {
        let path = format!("/v1/transfers/{}/exchange/authorize", transaction_id.into());
        let body = json!({ "otp": otp.into() });
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn convert_money(
        &self,
        from_amount: f64,
        from_currency: impl Into<String>,
        to_currency: impl Into<String>,
    ) -> Result<ConvertMoneyResponse> {
        let body = json!({
            "fromAmount": from_amount,
            "fromCurrency": from_currency.into(),
            "toCurrency": to_currency.into(),
        });
        let response = self.client.post("/v1/transfers/convert", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_exchange_rates(&self) -> Result<FetchExchangeRatesResponse> {
        let response = self.client.get("/v1/transfers/rates", None).await?;
        Ok(serde_json::from_value(response)?)
    }
}