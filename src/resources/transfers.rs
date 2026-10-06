use crate::error::Result;
use crate::http_client::BlockingNombaClient;
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

    /// Fetches all supported Nigerian bank codes and names.
    ///
    /// Calls `GET /v1/transfers/banks`. Cache this response — bank codes
    /// rarely change. Use the `code` field as `bankCode` in transfer and
    /// account lookup requests.
    pub fn fetch_bank_codes(&self) -> Result<FetchBankCodesResponse> {
        let response = self.client.get("/v1/transfers/banks", None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Verifies a recipient bank account number before transferring.
    ///
    /// Calls `POST /v1/transfers/bank/lookup`. Returns the account holder's
    /// name. Always call this before a bank transfer so users can confirm
    /// the recipient.
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
        let response = self.client.post("/v2/transfers/bank", &body, None)?;
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
        let path = format!("/v2/transfers/bank/{}", account_id.into());
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
        let response = self.client.post("/v2/transfers/wallet", &body, None)?;
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
        let path = format!("/v2/transfers/wallet/{}", account_id.into());
        let response = self.client.post(&path, &body, None)?;
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

    /// Fetches all supported Nigerian bank codes and names.
    ///
    /// See [`Transfers::fetch_bank_codes`] for details.
    pub async fn fetch_bank_codes(&self) -> Result<FetchBankCodesResponse> {
        let response = self.client.get("/v1/transfers/banks", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Verifies a recipient bank account number before transferring.
    ///
    /// See [`Transfers::bank_account_lookup`] for details.
    pub async fn bank_account_lookup(
        &self,
        account_number: impl Into<String>,
        bank_code: impl Into<String>,
    ) -> Result<BankAccountLookupResponse> {
        let body = json!({
            "accountNumber": account_number.into(),
            "bankCode": bank_code.into(),
        });
        let response = self
            .client
            .post("/v1/transfers/bank/lookup", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
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
        let response = self.client.post("/v2/transfers/bank", &body, None).await?;
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
        let path = format!("/v2/transfers/bank/{}", account_id.into());
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
        let response = self
            .client
            .post("/v2/transfers/wallet", &body, None)
            .await?;
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
        let path = format!("/v2/transfers/wallet/{}", account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}
