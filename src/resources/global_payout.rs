//! Global Payout resource for Nomba API
//!
//! Provides methods for cross-border payments, currency exchange, and wallet management.

use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Synchronous Global Payout client.
#[derive(Clone)]
pub struct GlobalPayout {
    client: BlockingNombaClient,
}

impl GlobalPayout {
    /// Creates a new `GlobalPayout` resource.
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    /// Fetches all currency wallets under the parent account.
    ///
    /// Returns a list of wallets with their balances (e.g., CAD, USD, CDF).
    /// Use this to check non-NGN balances instead of the NGN-only parent account endpoint.
    ///
    /// # Returns
    /// A [`FetchGlobalPayoutAccountsResponse`] containing the list of wallets.
    ///
    /// # Example
    /// ```no_run
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let accounts = nomba.global_payout.fetch_accounts()?;
    /// for wallet in accounts.data {
    ///     println!("Wallet: {} ({}) - Balance: {:?}", wallet.name, wallet.currency, wallet.balance);
    /// }
    /// # Ok::<(), nomba::NombaError>(())
    /// ```
    pub fn fetch_accounts(&self) -> Result<FetchGlobalPayoutAccountsResponse> {
        let response = self.client.get("/v1/global-payout/accounts", None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches a specific wallet by its account ID.
    ///
    /// Note: The `account_id` is the wallet ID (e.g., "66bc8c0e054dfe06b69a840a"),
    /// NOT a currency code like "CAD" or "USD". Passing a currency code returns 404.
    ///
    /// # Arguments
    /// * `account_id` - The wallet ID as returned by `fetch_accounts`
    ///
    /// # Returns
    /// A [`FetchGlobalPayoutAccountResponse`] containing the wallet details.
    ///
    /// # Example
    /// ```no_run
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let wallet = nomba.global_payout.fetch_account("66bc8c0e054dfe06b69a840a")?;
    /// println!("Wallet: {} - Available: {:?}", wallet.data.name, wallet.data.available_balance);
    /// # Ok::<(), nomba::NombaError>(())
    /// ```
    pub fn fetch_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<FetchGlobalPayoutAccountResponse> {
        let path = format!("/v1/global-payout/accounts/{}", account_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Authorizes a cross-border transfer.
    ///
    /// Supports multiple payment methods: BANK, MobileMoney, INTERAC, FASTER_PAYMENTS, SEPA, ACH, WIRE.
    /// The required fields vary by payment method.
    ///
    /// # Arguments
    /// * `amount` - Transfer amount
    /// * `source_currency` - Source currency (e.g., "USD")
    /// * `destination_currency` - Destination currency (e.g., "CDF")
    /// * `receiver_name` - Beneficiary name
    /// * `source_country_iso_code` - Source country ISO code (e.g., "US")
    /// * `destination_country_iso_code` - Destination country ISO code (e.g., "CD")
    /// * `payment_method` - Payment rail: "BANK", "MobileMoney", "INTERAC", "FASTER_PAYMENTS", "SEPA", "ACH", "WIRE"
    /// * `account_type` - Account type: "INDIVIDUAL" or "CORPORATE"
    /// * `account_number` - Account number, IBAN, or phone number (required for BANK, MobileMoney, FASTER_PAYMENTS, SEPA)
    /// * `institution_code` - Routing code / SWIFT / BIC / institution number (method-dependent)
    /// * `institution_name` - Bank/provider display name (method-dependent)
    /// * `bank_account_type` - "CHECKING" or "SAVINGS" (required for ACH, WIRE)
    /// * `purpose_of_payment` - Reason for transfer (required for ACH, WIRE, SEPA)
    /// * `narration` - Optional description
    /// * `locked_exchange_rate_id` - Optional locked rate from `convert_money`
    /// * `bank_address` - Bank address (required for WIRE)
    /// * `bank_city` - Bank city (required for WIRE)
    /// * `bank_state` - Bank state (required for WIRE)
    /// * `bank_zip_code` - Bank ZIP code (required for WIRE)
    /// * `beneficiary` - Optional [`Beneficiary`] with INTERAC-specific fields
    ///
    /// # Returns
    /// An [`AuthorizeTransferResponse`] with transaction details.
    ///
    /// # Example
    /// ```no_run
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let transfer = nomba.global_payout.authorize_transfer(
    ///     500.0, "USD".to_string(), "CDF".to_string(), "John Doe".to_string(),
    ///     "US".to_string(), "CD".to_string(), "MobileMoney".to_string(), "INDIVIDUAL".to_string(),
    ///     Some("0903086112".to_string()), None, None, None, None,
    ///     Some("Family support".to_string()), None, None, None, None, None, None,
    /// )?;
    /// println!("Transfer: {}", transfer.data.wt_transaction_id);
    /// # Ok::<(), nomba::NombaError>(())
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn authorize_transfer(
        &self,
        amount: f64,
        source_currency: impl Into<String>,
        destination_currency: impl Into<String>,
        receiver_name: impl Into<String>,
        source_country_iso_code: impl Into<String>,
        destination_country_iso_code: impl Into<String>,
        payment_method: impl Into<String>,
        account_type: impl Into<String>,
        account_number: Option<String>,
        institution_code: Option<String>,
        institution_name: Option<String>,
        bank_account_type: Option<String>,
        purpose_of_payment: Option<String>,
        narration: Option<String>,
        locked_exchange_rate_id: Option<String>,
        bank_address: Option<String>,
        bank_city: Option<String>,
        bank_state: Option<String>,
        bank_zip_code: Option<String>,
        beneficiary: Option<Beneficiary>,
    ) -> Result<AuthorizeTransferResponse> {
        let mut body = json!({
            "amount": amount,
            "sourceCurrency": source_currency.into(),
            "destinationCurrency": destination_currency.into(),
            "receiverName": receiver_name.into(),
            "sourceCountryIsoCode": source_country_iso_code.into(),
            "destinationCountryIsoCode": destination_country_iso_code.into(),
            "paymentMethod": payment_method.into(),
            "accountType": account_type.into(),
        });
        if let Some(v) = account_number {
            body["accountNumber"] = json!(v);
        }
        if let Some(v) = institution_code {
            body["institutionCode"] = json!(v);
        }
        if let Some(v) = institution_name {
            body["institutionName"] = json!(v);
        }
        if let Some(v) = bank_account_type {
            body["bankAccountType"] = json!(v);
        }
        if let Some(v) = purpose_of_payment {
            body["purposeOfPayment"] = json!(v);
        }
        if let Some(v) = narration {
            body["narration"] = json!(v);
        }
        if let Some(v) = locked_exchange_rate_id {
            body["lockedExchangeRateId"] = json!(v);
        }
        if let Some(v) = bank_address {
            body["bankAddress"] = json!(v);
        }
        if let Some(v) = bank_city {
            body["bankCity"] = json!(v);
        }
        if let Some(v) = bank_state {
            body["bankState"] = json!(v);
        }
        if let Some(v) = bank_zip_code {
            body["bankZipCode"] = json!(v);
        }
        if let Some(v) = beneficiary {
            body["beneficiary"] = json!(v);
        }
        let response = self.client.post("/v1/global-payout/transfer/authorize", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Authorizes a currency exchange between your own wallets.
    ///
    /// Transfers funds between your wallets in different currencies.
    ///
    /// # Arguments
    /// * `amount` - Amount to exchange
    /// * `source_currency` - Source currency (e.g., "USD")
    /// * `destination_currency` - Destination currency (e.g., "CDF")
    /// * `sender_name` - Sender name
    /// * `receiver_name` - Receiver name
    /// * `source_country_iso_code` - Source country ISO code
    /// * `destination_country_iso_code` - Destination country ISO code
    /// * `narration` - Optional description
    /// * `locked_exchange_rate_id` - Optional locked rate from `fetch_exchange_rates`
    ///
    /// # Returns
    /// An [`AuthorizeExchangeResponse`] with transaction details.
    ///
    /// # Example
    /// ```no_run
    /// use nomba::Nomba;
    ///
    /// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
    /// let exchange = nomba.global_payout.authorize_exchange(
    ///     1000.0, "USD".to_string(), "CDF".to_string(), "John Doe".to_string(), "John Doe".to_string(),
    ///     "US".to_string(), "CD".to_string(), Some("Salary conversion".to_string()), None,
    /// )?;
    /// println!("Exchange: {}", exchange.data.wt_transaction_id);
    /// # Ok::<(), nomba::NombaError>(())
    /// ```
    pub fn authorize_exchange(
        &self,
        amount: f64,
        source_currency: impl Into<String>,
        destination_currency: impl Into<String>,
        sender_name: impl Into<String>,
        receiver_name: impl Into<String>,
        source_country_iso_code: impl Into<String>,
        destination_country_iso_code: impl Into<String>,
        narration: Option<String>,
        locked_exchange_rate_id: Option<String>,
    ) -> Result<AuthorizeExchangeResponse> {
        let mut body = json!({
            "amount": amount,
            "sourceCurrency": source_currency.into(),
            "destinationCurrency": destination_currency.into(),
            "senderName": sender_name.into(),
            "receiverName": receiver_name.into(),
            "sourceCountryIsoCode": source_country_iso_code.into(),
            "destinationCountryIsoCode": destination_country_iso_code.into(),
        });
        if let Some(v) = narration {
            body["narration"] = json!(v);
        }
        if let Some(v) = locked_exchange_rate_id {
            body["lockedExchangeRateId"] = json!(v);
        }
        let response = self.client.post("/v1/global-payout/exchange/authorize", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches a global payout transaction by ID.
    ///
    /// # Arguments
    /// * `transaction_id` - The transaction ID (wtTransactionId)
    ///
    /// # Returns
    /// A [`FetchGlobalPayoutTransactionResponse`] with transaction details.
    pub fn fetch_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchGlobalPayoutTransactionResponse> {
        let path = format!("/v1/global-payout/transactions/{}", transaction_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

/// Asynchronous Global Payout client.
#[derive(Clone)]
pub struct AsyncGlobalPayout {
    client: NombaClient,
}

impl AsyncGlobalPayout {
    /// Creates a new `AsyncGlobalPayout` resource.
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    /// Fetches all currency wallets under the parent account.
    ///
    /// Returns a list of wallets with their balances (e.g., CAD, USD, CDF).
    pub async fn fetch_accounts(&self) -> Result<FetchGlobalPayoutAccountsResponse> {
        let response = self.client.get("/v1/global-payout/accounts", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches a specific wallet by its account ID.
    ///
    /// Note: The `account_id` is the wallet ID, NOT a currency code.
    pub async fn fetch_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<FetchGlobalPayoutAccountResponse> {
        let path = format!("/v1/global-payout/accounts/{}", account_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Authorizes a cross-border transfer.
    ///
    /// See [`GlobalPayout::authorize_transfer`] for full parameter documentation.
    #[allow(clippy::too_many_arguments)]
    pub async fn authorize_transfer(
        &self,
        amount: f64,
        source_currency: impl Into<String>,
        destination_currency: impl Into<String>,
        receiver_name: impl Into<String>,
        source_country_iso_code: impl Into<String>,
        destination_country_iso_code: impl Into<String>,
        payment_method: impl Into<String>,
        account_type: impl Into<String>,
        account_number: Option<String>,
        institution_code: Option<String>,
        institution_name: Option<String>,
        bank_account_type: Option<String>,
        purpose_of_payment: Option<String>,
        narration: Option<String>,
        locked_exchange_rate_id: Option<String>,
        bank_address: Option<String>,
        bank_city: Option<String>,
        bank_state: Option<String>,
        bank_zip_code: Option<String>,
        beneficiary: Option<Beneficiary>,
    ) -> Result<AuthorizeTransferResponse> {
        let mut body = json!({
            "amount": amount,
            "sourceCurrency": source_currency.into(),
            "destinationCurrency": destination_currency.into(),
            "receiverName": receiver_name.into(),
            "sourceCountryIsoCode": source_country_iso_code.into(),
            "destinationCountryIsoCode": destination_country_iso_code.into(),
            "paymentMethod": payment_method.into(),
            "accountType": account_type.into(),
        });
        if let Some(v) = account_number {
            body["accountNumber"] = json!(v);
        }
        if let Some(v) = institution_code {
            body["institutionCode"] = json!(v);
        }
        if let Some(v) = institution_name {
            body["institutionName"] = json!(v);
        }
        if let Some(v) = bank_account_type {
            body["bankAccountType"] = json!(v);
        }
        if let Some(v) = purpose_of_payment {
            body["purposeOfPayment"] = json!(v);
        }
        if let Some(v) = narration {
            body["narration"] = json!(v);
        }
        if let Some(v) = locked_exchange_rate_id {
            body["lockedExchangeRateId"] = json!(v);
        }
        if let Some(v) = bank_address {
            body["bankAddress"] = json!(v);
        }
        if let Some(v) = bank_city {
            body["bankCity"] = json!(v);
        }
        if let Some(v) = bank_state {
            body["bankState"] = json!(v);
        }
        if let Some(v) = bank_zip_code {
            body["bankZipCode"] = json!(v);
        }
        if let Some(v) = beneficiary {
            body["beneficiary"] = json!(v);
        }
        let response = self.client.post("/v1/global-payout/transfer/authorize", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Authorizes a currency exchange between your own wallets.
    ///
    /// See [`GlobalPayout::authorize_exchange`] for full parameter documentation.
    pub async fn authorize_exchange(
        &self,
        amount: f64,
        source_currency: impl Into<String>,
        destination_currency: impl Into<String>,
        sender_name: impl Into<String>,
        receiver_name: impl Into<String>,
        source_country_iso_code: impl Into<String>,
        destination_country_iso_code: impl Into<String>,
        narration: Option<String>,
        locked_exchange_rate_id: Option<String>,
    ) -> Result<AuthorizeExchangeResponse> {
        let mut body = json!({
            "amount": amount,
            "sourceCurrency": source_currency.into(),
            "destinationCurrency": destination_currency.into(),
            "senderName": sender_name.into(),
            "receiverName": receiver_name.into(),
            "sourceCountryIsoCode": source_country_iso_code.into(),
            "destinationCountryIsoCode": destination_country_iso_code.into(),
        });
        if let Some(v) = narration {
            body["narration"] = json!(v);
        }
        if let Some(v) = locked_exchange_rate_id {
            body["lockedExchangeRateId"] = json!(v);
        }
        let response = self.client.post("/v1/global-payout/exchange/authorize", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Fetches a global payout transaction by ID.
    pub async fn fetch_transaction(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchGlobalPayoutTransactionResponse> {
        let path = format!("/v1/global-payout/transactions/{}", transaction_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}

/// Beneficiary details for INTERAC and other payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beneficiary {
    /// Beneficiary email (for INTERAC)
    #[serde(rename = "beneficiaryEmail")]
    pub beneficiary_email: Option<String>,
    /// Security question (for INTERAC)
    #[serde(rename = "securityQuestion")]
    pub security_question: Option<String>,
    /// Security question answer (for INTERAC)
    #[serde(rename = "securityQuestionAnswer")]
    pub security_question_answer: Option<String>,
    /// Transit number (for INTERAC)
    #[serde(rename = "transitNumber")]
    pub transit_number: Option<String>,
    /// Beneficiary address
    #[serde(rename = "beneficiaryAddress")]
    pub beneficiary_address: Option<String>,
    /// Beneficiary city
    #[serde(rename = "beneficiaryCity")]
    pub beneficiary_city: Option<String>,
    /// Beneficiary state
    #[serde(rename = "beneficiaryState")]
    pub beneficiary_state: Option<String>,
    /// Beneficiary postal code
    #[serde(rename = "beneficiaryPostCode")]
    pub beneficiary_post_code: Option<String>,
}