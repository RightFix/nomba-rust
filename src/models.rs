// Generated models for Nomba API responses
// These mirror the Python models.py structure

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NombaResponse<T> {
    pub code: String,
    pub description: String,
    pub data: T,
}

/// Paginated response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub results: Vec<T>,
    pub cursor: Option<String>,
}

/// Account models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccountData {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountHolderId")]
    pub account_holder_id: String,
    #[serde(rename = "accountRef")]
    pub account_ref: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub email: String,
    pub bvn: String,
    pub status: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    pub currency: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    #[serde(rename = "expiryDate")]
    pub expiry_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubAccountResponse {
    pub code: String,
    pub description: String,
    pub data: SubAccountData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDetailsData {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "accountHolderId")]
    pub account_holder_id: String,
    #[serde(rename = "accountRef")]
    pub account_ref: String,
    pub bvn: String,
    pub status: String,
    #[serde(rename = "type")]
    pub account_type: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    pub currency: String,
    pub banks: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchAccountDetailsResponse {
    pub code: String,
    pub description: String,
    pub data: AccountDetailsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalanceData {
    pub amount: String,
    pub currency: String,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchAccountBalanceResponse {
    pub code: String,
    pub description: String,
    pub data: AccountBalanceData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendAccountResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactivateAccountResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalAssignmentData {
    #[serde(rename = "terminalId")]
    pub terminal_id: String,
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "parentAccountId")]
    pub parent_account_id: String,
    #[serde(rename = "merchantName")]
    pub merchant_name: String,
    #[serde(rename = "terminalLabel")]
    pub terminal_label: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignTerminalResponse {
    pub code: String,
    pub description: String,
    pub data: TerminalAssignmentData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignTerminalResponse {
    pub code: String,
    pub description: String,
    pub data: TerminalAssignmentData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccountAccessResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Virtual Account models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAccountData {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "accountHolderId")]
    pub account_holder_id: String,
    #[serde(rename = "accountRef")]
    pub account_ref: String,
    pub bvn: String,
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "bankName")]
    pub bank_name: String,
    #[serde(rename = "bankAccountNumber")]
    pub bank_account_number: String,
    #[serde(rename = "bankAccountName")]
    pub bank_account_name: String,
    pub currency: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub expired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVirtualAccountResponse {
    pub code: String,
    pub description: String,
    pub data: VirtualAccountData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterVirtualAccountsResponse {
    pub code: String,
    pub description: String,
    pub data: PaginatedResponse<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVirtualAccountResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchVirtualAccountResponse {
    pub code: String,
    pub description: String,
    pub data: VirtualAccountData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpireVirtualAccountResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Checkout models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutOrderData {
    #[serde(rename = "checkoutLink")]
    pub checkout_link: String,
    #[serde(rename = "orderReference")]
    pub order_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutOrderResponse {
    pub code: String,
    pub description: String,
    pub data: CheckoutOrderData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutTransactionData {
    pub success: bool,
    pub message: String,
    pub order: HashMap<String, serde_json::Value>,
    #[serde(rename = "transactionDetails")]
    pub transaction_details: HashMap<String, serde_json::Value>,
    #[serde(rename = "transferDetails")]
    pub transfer_details: HashMap<String, serde_json::Value>,
    #[serde(rename = "cardDetails")]
    pub card_details: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchCheckoutTransactionResponse {
    pub code: String,
    pub description: String,
    pub data: CheckoutTransactionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundCheckoutResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutOrderDetailsData {
    pub order: HashMap<String, serde_json::Value>,
    #[serde(rename = "hasSavedCards")]
    pub has_saved_cards: bool,
    #[serde(rename = "base64EncodedRsaPublicKey")]
    pub base64_encoded_rsa_public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchCheckoutOrderDetailsResponse {
    pub code: String,
    pub description: String,
    pub data: CheckoutOrderDetailsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitCardDetailsResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitOtpResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendOtpResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchCheckoutTransactionDetailsResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchFlashAccountResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestUserOtpResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitUserOtpResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchUserSavedCardsResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelCheckoutTransactionResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelCheckoutOrderResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Charge models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizedCardData {
    pub status: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeWithTokenizedCardResponse {
    pub code: String,
    pub description: String,
    pub data: TokenizedCardData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizedCardListData {
    #[serde(rename = "nextPage")]
    pub next_page: String,
    #[serde(rename = "tokenizedCardDataList")]
    pub tokenized_card_data_list: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTokenizedCardsResponse {
    pub code: String,
    pub description: String,
    pub data: TokenizedCardListData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTokenizedCardResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTokenizedCardResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchBankCodesResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountLookupResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Transfers models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferData {
    pub amount: String,
    pub source: String,
    #[serde(rename = "sourceUserId")]
    pub source_user_id: String,
    #[serde(rename = "customerBillerId")]
    pub customer_biller_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub fee: f64,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    pub id: String,
    #[serde(rename = "type")]
    pub transfer_type: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformBankTransferResponse {
    pub code: String,
    pub description: String,
    pub data: BankTransferData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransferData {
    pub amount: f64,
    pub meta: HashMap<String, serde_json::Value>,
    pub fee: f64,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    pub id: String,
    #[serde(rename = "type")]
    pub transfer_type: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformWalletTransferResponse {
    pub code: String,
    pub description: String,
    pub data: WalletTransferData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertMoneyData {
    #[serde(rename = "fromAmount")]
    pub from_amount: f64,
    #[serde(rename = "fromCurrency")]
    pub from_currency: String,
    #[serde(rename = "fromFormatted")]
    pub from_formatted: String,
    #[serde(rename = "toAmount")]
    pub to_amount: f64,
    #[serde(rename = "toCurrency")]
    pub to_currency: String,
    #[serde(rename = "toFormatted")]
    pub to_formatted: String,
    #[serde(rename = "spreadAmount")]
    pub spread_amount: f64,
    #[serde(rename = "spreadCurrency")]
    pub spread_currency: String,
    #[serde(rename = "exchangeRateId")]
    pub exchange_rate_id: String,
    #[serde(rename = "currencyPairName")]
    pub currency_pair_name: String,
    #[serde(rename = "feeAmount")]
    pub fee_amount: f64,
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertMoneyResponse {
    pub code: String,
    pub description: String,
    pub data: ConvertMoneyData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchExchangeRatesResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Terminals models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPaymentRequestData {
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    pub status: String,
    pub amount: f64,
    pub currency: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPaymentRequestResponse {
    pub code: String,
    pub description: String,
    pub data: SendPaymentRequestData,
}

/// Transactions models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub id: String,
    pub status: String,
    pub amount: f64,
    #[serde(rename = "fixedCharge")]
    pub fixed_charge: f64,
    pub source: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    #[serde(rename = "gatewayMessage")]
    pub gateway_message: String,
    #[serde(rename = "customerBillerId")]
    pub customer_biller_id: String,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "posTid")]
    pub pos_tid: String,
    #[serde(rename = "terminalId")]
    pub terminal_id: String,
    #[serde(rename = "providerTerminalId")]
    pub provider_terminal_id: String,
    pub rrn: String,
    #[serde(rename = "posSerialNumber")]
    pub pos_serial_number: String,
    #[serde(rename = "posTerminalLabel")]
    pub pos_terminal_label: String,
    pub stan: String,
    #[serde(rename = "paymentVendorReference")]
    pub payment_vendor_reference: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "posRrn")]
    pub pos_rrn: String,
    #[serde(rename = "merchantTxRef")]
    pub merchant_tx_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchTransactionResponse {
    pub code: String,
    pub description: String,
    pub data: TransactionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterTransactionsResponse {
    pub code: String,
    pub description: String,
    pub data: PaginatedResponse<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmTransactionBySessionResponse {
    pub code: String,
    pub description: String,
    pub data: TransactionData,
}

/// Airtime & Data models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirtimePurchaseData {
    pub amount: f64,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirtimePurchaseResponse {
    pub code: String,
    pub description: String,
    pub data: AirtimePurchaseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataVendingResponse {
    pub code: String,
    pub description: String,
    pub data: AirtimePurchaseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchDataPlansResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// CableTV models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableTvLookupResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableTvSubscriptionData {
    pub amount: f64,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub status: String,
    pub id: String,
    pub fee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableTvSubscriptionResponse {
    pub code: String,
    pub description: String,
    pub data: CableTvSubscriptionData,
}

/// Electricity models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricityProviderData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchElectricityProvidersResponse {
    pub code: String,
    pub description: String,
    pub data: ElectricityProviderData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricityCustomerLookupResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendElectricityData {
    pub amount: f64,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub status: String,
    pub id: String,
    pub fee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendElectricityResponse {
    pub code: String,
    pub description: String,
    pub data: VendElectricityData,
}

/// Betting models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchBettingProvidersResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BettingCustomerLookupResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendBettingData {
    pub amount: f64,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub status: String,
    pub id: String,
    pub fee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendBettingResponse {
    pub code: String,
    pub description: String,
    pub data: VendBettingData,
}

/// Direct Debits models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandateData {
    pub items: HashMap<String, serde_json::Value>,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    #[serde(rename = "totalItems")]
    pub total_items: u32,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMandatesResponse {
    pub code: String,
    pub description: String,
    pub data: MandateData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMandateStatusResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebitMandateResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandateStatusData {
    #[serde(rename = "customerAccountName")]
    pub customer_account_name: String,
    #[serde(rename = "mandateId")]
    pub mandate_id: String,
    #[serde(rename = "customerAccountNumber")]
    pub customer_account_number: String,
    #[serde(rename = "mandateStatus")]
    pub mandate_status: String,
    #[serde(rename = "rejectionComment")]
    pub rejection_comment: String,
    #[serde(rename = "mandateAdviceStatus")]
    pub mandate_advice_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMandateStatusResponse {
    pub code: String,
    pub description: String,
    pub data: MandateStatusData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandateDetailData {
    pub status: String,
    #[serde(rename = "customerAccountNumber")]
    pub customer_account_number: String,
    #[serde(rename = "customerAccountName")]
    pub customer_account_name: String,
    #[serde(rename = "bankCode")]
    pub bank_code: String,
    pub amount: f64,
    #[serde(rename = "customerName")]
    pub customer_name: String,
    #[serde(rename = "customerAddress")]
    pub customer_address: String,
    #[serde(rename = "customerEmail")]
    pub customer_email: String,
    #[serde(rename = "customerPhoneNumber")]
    pub customer_phone_number: String,
    #[serde(rename = "merchantReference")]
    pub merchant_reference: String,
    pub frequency: String,
    #[serde(rename = "startDate")]
    pub start_date: Vec<i32>,
    #[serde(rename = "endDate")]
    pub end_date: Vec<i32>,
    #[serde(rename = "mandateAdviceStatus")]
    pub mandate_advice_status: String,
    #[serde(rename = "mandateId")]
    pub mandate_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMandateByIdResponse {
    pub code: String,
    pub description: String,
    pub data: MandateDetailData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMandateResponse {
    #[serde(rename = "responseMessage")]
    pub response_message: String,
    #[serde(rename = "responseCode")]
    pub response_code: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Global Collections models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateMobileMoneyInflowData {
    #[serde(rename = "transactionReference")]
    pub transaction_reference: String,
    pub status: String,
    pub message: String,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateMobileMoneyInflowResponse {
    pub code: String,
    pub description: String,
    pub data: InitiateMobileMoneyInflowData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchCollectionTransactionData {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "coreUserId")]
    pub core_user_id: String,
    pub account: String,
    pub status: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchCollectionTransactionResponse {
    pub code: String,
    pub description: String,
    pub data: FetchCollectionTransactionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchDrcInflowProvidersResponse {
    pub code: String,
    pub description: String,
    pub data: HashMap<String, serde_json::Value>,
}

/// Global Payout models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPayoutTransactionData {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub status: String,
    #[serde(rename = "coreStatus")]
    pub core_status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchGlobalPayoutTransactionResponse {
    pub code: String,
    pub description: String,
    pub data: GlobalPayoutTransactionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPayoutAccountData {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub name: String,
    pub currency: String,
    pub balance: Option<f64>,
    #[serde(rename = "availableBalance")]
    pub available_balance: Option<f64>,
    pub status: String,
    #[serde(rename = "operationRegion")]
    pub operation_region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchGlobalPayoutAccountsResponse {
    pub code: String,
    pub description: String,
    pub status: bool,
    pub data: Vec<GlobalPayoutAccountData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchGlobalPayoutAccountResponse {
    pub code: String,
    pub description: String,
    pub status: bool,
    pub data: GlobalPayoutAccountData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeTransferMeta {
    #[serde(rename = "source_amount")]
    pub source_amount: String,
    #[serde(rename = "destination_amount")]
    pub destination_amount: String,
    #[serde(rename = "source_currency")]
    pub source_currency: String,
    #[serde(rename = "destination_currency")]
    pub destination_currency: String,
    #[serde(rename = "amount_charged")]
    pub amount_charged: String,
    #[serde(rename = "currency_pair_name")]
    pub currency_pair_name: String,
    #[serde(rename = "payment_method")]
    pub payment_method: String,
    #[serde(rename = "destination_country")]
    pub destination_country: String,
    #[serde(rename = "destination_country_name")]
    pub destination_country_name: String,
    #[serde(rename = "source_country")]
    pub source_country: String,
    pub narration: String,
    #[serde(rename = "trade_side")]
    pub trade_side: String,
    #[serde(rename = "spread_amount")]
    pub spread_amount: String,
    #[serde(rename = "spread_currency")]
    pub spread_currency: String,
    #[serde(rename = "wt_transaction_id")]
    pub wt_transaction_id: String,
    #[serde(rename = "trade_context")]
    pub trade_context: String,
    #[serde(rename = "transactionCategory")]
    pub transaction_category: String,
    #[serde(rename = "payment_destination_type")]
    pub payment_destination_type: String,
    #[serde(rename = "tradeType")]
    pub trade_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeTransferData {
    #[serde(rename = "wtTransactionId")]
    pub wt_transaction_id: String,
    #[serde(rename = "coreTransactionId")]
    pub core_transaction_id: String,
    pub status: String,
    #[serde(rename = "coreStatus")]
    pub core_status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    #[serde(rename = "prettyStatus")]
    pub pretty_status: String,
    pub meta: AuthorizeTransferMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeTransferResponse {
    pub code: String,
    pub description: String,
    pub data: AuthorizeTransferData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeExchangeData {
    #[serde(rename = "wtTransactionId")]
    pub wt_transaction_id: String,
    #[serde(rename = "coreTransactionId")]
    pub core_transaction_id: String,
    pub status: String,
    #[serde(rename = "coreStatus")]
    pub core_status: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizeExchangeResponse {
    pub code: String,
    pub description: String,
    pub data: AuthorizeExchangeData,
}

/// Auth models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeTokenResponse {
    pub code: String,
    pub description: String,
}

/// CableTV models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CableTvPlanData {
    #[serde(rename = "subScriptionType")]
    pub subscription_type: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchCableTvPlansResponse {
    pub code: String,
    pub description: String,
    pub data: Vec<CableTvPlanData>,
    pub message: String,
}
