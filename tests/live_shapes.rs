//! Live-shape conformance tests: example payloads taken from the Nomba
//! API documentation must deserialize into the SDK's response structs.
//!
//! Fixtures live in `tests/fixtures/`. These tests run offline.

use nomba_rs::models::*;

fn load(name: &str) -> serde_json::Value {
    let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
    serde_json::from_str(&std::fs::read_to_string(path).expect("fixture missing"))
        .expect("invalid fixture json")
}

#[test]
fn authorize_transfer_shape() {
    let v = load("authorize_transfer.json");
    let resp: AuthorizeTransferResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.wt_transaction_id, "01kky197w3xc6wyjenpc5r0tnp");
    assert_eq!(
        resp.data.idempotency_key.as_deref(),
        Some("5e246f9a-3e79-4b11-92f1-dbd98b53b308")
    );
    assert_eq!(resp.data.idempotent_replay, Some(false));
    assert_eq!(resp.data.meta.payment_method, "MobileMoney");
}

#[test]
fn authorize_exchange_shape() {
    let v = load("authorize_exchange.json");
    let resp: AuthorizeExchangeResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.transaction_type, "EXCHANGE");
    assert_eq!(resp.data.idempotent_replay, Some(false));
}

#[test]
fn v2_airtime_shape() {
    let v = load("v2_airtime.json");
    let resp: AirtimePurchaseResponse = serde_json::from_value(v).unwrap();
    assert_eq!(
        resp.data.id.as_deref(),
        Some("API-TOPUP-A6950-8c060b7c-4155-4e33-a846-101b49229c33")
    );
    assert_eq!(resp.data.customer_biller_id.as_deref(), Some("08123456788"));
    assert!(resp.data.source_user_id.is_none());
}

#[test]
fn electricity_lookup_v2_shape() {
    let v = load("electricity_lookup_v2.json");
    let resp: ElectricityCustomerLookupResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.name, "John Doe");
    assert!(resp.data.address.unwrap().contains("Lagos"));
}

#[test]
fn cabletv_lookup_string_shape() {
    let v = load("cabletv_lookup.json");
    let resp: CableTvLookupResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.name(), Some("John Doe"));
}

#[test]
fn exchange_rates_shape() {
    let v = load("exchange_rates.json");
    let resp: FetchExchangeRatesResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.rates.len(), 1);
    assert_eq!(
        resp.data.rates[0].currency_pair_name.as_deref(),
        Some("EUR/USD")
    );
}

#[test]
fn collection_providers_shape() {
    let v = load("collection_providers.json");
    let resp: FetchDrcInflowProvidersResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].code.as_deref(), Some("MPESA"));
}

#[test]
fn bank_codes_shape() {
    let v = load("bank_codes.json");
    let resp: FetchBankCodesResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data[0].nip_code.as_deref(), Some("000005"));
}

#[test]
fn bank_lookup_shape() {
    let v = load("bank_lookup.json");
    let resp: BankAccountLookupResponse = serde_json::from_value(v).unwrap();
    assert_eq!(resp.data.account_name.as_deref(), Some("M.A Animashaun"));
}

#[test]
fn data_plans_shape() {
    let v = load("data_plans.json");
    let resp: FetchDataPlansResponse = serde_json::from_value(v).unwrap();
    let plans = resp.data.as_list().expect("plans as list");
    assert_eq!(plans.len(), 1);
    let plan: DataPlan = serde_json::from_value(plans[0].clone()).unwrap();
    assert_eq!(plan.product_id.as_deref(), Some("mtn47"));
}
