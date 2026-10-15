# nomba-rs

[![License: AGPL-3.0-only](https://img.shields.io/badge/license-AGPL--3.0--only-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/nomba-rs.svg)](https://crates.io/crates/nomba-rs)

Unofficial async Rust SDK for the [Nomba](https://developer.nomba.com) payments API.
Full documentation (guides + API reference): <https://RightFix.github.io/nomba-docs-rust/>

## Features

- **Async client** - Tokio-based `async`/`await` API covering every endpoint group
- **Complete API coverage** - Accounts, Virtual Accounts, Checkout, Charge, Transfers, Terminals, Transactions, Airtime/Data, CableTV, Electricity, Betting, Direct Debits, Global Collections, Global Payout, Auth
- **OAuth2 authentication** - Automatic token management with refresh
- **Retry logic** - Automatic retry with exponential backoff for 429/5xx errors
- **Webhook verification** - HMAC-SHA256 signature verification with replay protection
- **Card payment flows** - Guided multi-step card payment (OTP, 3DS)
- **Pagination helpers** - Cursor-based pagination as a `Stream`
- **Bounded concurrency** - Rate-limited concurrent request execution
- **Request validation** - Optional local validation against the bundled OpenAPI spec

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nomba-rs = "0.5"
tokio = { version = "1", features = ["full"] }
```

Or for local development:
```toml
[dependencies]
nomba-rs = { path = "../nomba-rust" }
```

### Features

- `validation` - Enable local request validation against the bundled OpenAPI spec

```toml
nomba-rs = { version = "0.5", features = ["validation"] }
```

## Quick Start

```rust
use nomba_rs::Nomba;

#[tokio::main]
async fn main() -> nomba_rs::Result<()> {
    let nomba = Nomba::new(
        "your_client_id",
        "your_client_secret",
        "your_account_id",
    ).await?;

    // Create a virtual account (pass a BVN and/or NIN to create against it)
    let account = nomba.virtual_accounts.create_virtual_account(
        "ref-123",
        "Jane Doe",
        None, None, None, None, None
    ).await?;

    println!("Created account: {:?}", account.data.account_ref);
    Ok(())
}
```

### Sandbox Mode

```rust
let nomba = Nomba::new_sandbox(
    "your_client_id",
    "your_client_secret",
    "your_account_id",
).await?;
```

### Configuration

```rust
use nomba_rs::NombaClientConfig;
use std::time::Duration;

let config = NombaClientConfig::new(
    "client_id".into(),
    "client_secret".into(),
    "account_id".into(),
)
.sandbox(true)
.timeout(Duration::from_secs(60))
.max_retries(5)
.backoff_factor(1.0);

let nomba = Nomba::with_config(config).await?;
```

## Resource Modules

Each API group is exposed as a resource on the client:

| Resource | Field |
|----------|-------|
| Accounts | `nomba.accounts` |
| Virtual Accounts | `nomba.virtual_accounts` |
| Checkout | `nomba.checkout` |
| Charge | `nomba.charge` |
| Transfers | `nomba.transfers` |
| Terminals | `nomba.terminals` |
| Transactions | `nomba.transactions` |
| Airtime & Data | `nomba.airtime_data` |
| CableTV | `nomba.cabletv` |
| Electricity | `nomba.electricity` |
| Betting | `nomba.betting` |
| Direct Debits | `nomba.direct_debits` |
| Global Collections | `nomba.global_collections` |
| Global Payout | `nomba.global_payout` |
| Auth | `nomba.auth` |

## Auth - Token Management

```rust
use nomba_rs::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id").await?;

// Revoke an access token (e.g., on logout)
let revoked = nomba.auth.revoke_access_token("access_token_to_revoke").await?;
println!("Token revoked: {}", revoked.description);

// Refresh an expired access token
let refreshed = nomba.auth.refresh_access_token("refresh_token").await?;
println!("New token: {}", refreshed.data.access_token);
```

## Global Payout

```rust
use nomba_rs::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id").await?;

// Fetch all currency wallets
let accounts = nomba.global_payout.fetch_accounts().await?;
for wallet in accounts.data {
    println!("Wallet: {} ({}) - Balance: {:?}", wallet.name, wallet.currency, wallet.balance);
}

// Fetch a specific wallet by accountId (not currency code!)
let wallet = nomba.global_payout.fetch_account("66bc8c0e054dfe06b69a840a").await?;
println!("Wallet: {} - Available: {:?}", wallet.data.name, wallet.data.available_balance);

// Authorize a cross-border transfer (e.g., MobileMoney to DRC)
let transfer = nomba.global_payout.authorize_transfer(
    500.0,                          // amount
    "USD",                          // source_currency
    "CDF",                          // destination_currency
    "John Doe",                     // receiver_name
    "US",                           // source_country_iso_code
    "CD",                           // destination_country_iso_code
    "MobileMoney",                  // payment_method
    "INDIVIDUAL",                   // account_type
    Some("0903086112"),             // account_number
    None,                           // institution_code
    None,                           // institution_name
    None,                           // bank_account_type
    None,                           // purpose_of_payment
    Some("Family support"),         // narration
    None,                           // locked_exchange_rate_id
    None,                           // bank_address
    None,                           // bank_city
    None,                           // bank_state
    None,                           // bank_zip_code
    None,                           // beneficiary
    Some("5e246f9a-3e79-4b11-92f1-dbd98b53b308"), // idempotency_key
).await?;
println!("Transfer: {}", transfer.data.wt_transaction_id);

// Authorize an exchange between your own wallets
let exchange = nomba.global_payout.authorize_exchange(
    1000.0,                         // amount
    "USD",                          // source_currency
    "CDF",                          // destination_currency
    "John Doe",                     // sender_name
    "John Doe",                     // receiver_name
    "US",                           // source_country_iso_code
    "CD",                           // destination_country_iso_code
    Some("Salary conversion"),      // narration
    None,                           // locked_exchange_rate_id
    None,                           // idempotency_key
).await?;
println!("Exchange: {}", exchange.data.wt_transaction_id);
```

## CableTV - Fetch Plans

```rust
use nomba_rs::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id").await?;

// Get available DStv plans
let plans = nomba.cabletv.fetch_plans("dstv").await?;
for plan in plans.data {
    println!("Plan: {} - {:?}", plan.subscription_type, plan.amount);
}

// Also supports: gotv, startimes, ShowMax
```

## Airtime & Data - Data Vending with Product ID

```rust
use nomba_rs::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id").await?;

// 1. Fetch data plans to get product IDs
let plans = nomba.airtime_data.fetch_data_plans("MTN").await?;

// 2. Vend data using product_id (not amount!)
let vend = nomba.airtime_data.vend_data_parent(
    "mtn47",                        // product_id from plans response
    "08055441122",                  // phone_number
    "MTN",                          // network
    "txn-ref-123",                  // merchant_tx_ref
    Some("John Doe"),               // sender_name
).await?;
println!("Data vended: {}", vend.data.amount);
```

## Card Payment Flow

The SDK provides a guided flow for card payments:

```rust
use nomba_rs::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id").await?;

// 1. Create checkout order
let order = nomba.checkout.create_order(
    "order-123",
    "1000",
    "NGN",
    "customer@example.com",
    "John Doe",
    "https://example.com/redirect",
    None, None
).await?;

let order_ref = order.data.order_reference;

// 2. Start card payment flow
let mut flow = nomba.card_payment(order_ref);

// 3. Submit card details
let step = flow.submit_card(
    "encrypted_card_details",
    "rsa_public_key",
    Some(true),
    None
).await?;

if step.requires_otp {
    // 4. Submit OTP
    let step = flow.submit_otp("123456").await?;
}

if step.requires_3ds {
    // Redirect user using step.secure_authentication_data
}

// 5. Confirm final status
if step.completed {
    let result = flow.confirm().await?;
    println!("Payment successful: {:?}", result);
}
```

## Webhook Verification

```rust
use nomba_rs::webhooks::verify_webhook_request;
use std::collections::HashMap;

async fn handle_webhook(body: &[u8], headers: &HashMap<String, String>) -> nomba_rs::Result<()> {
    let payload = verify_webhook_request(
        "your_webhook_signature_key",
        body,
        headers,
        Some(300.0), // 5 min replay protection
    )?;

    // Process webhook payload
    println!("Received: {:?}", payload);
    Ok(())
}
```

## Pagination

`paginate` drives cursor pages as a `Stream`. It takes a `(limit, cursor)`
closure returning the raw page as `serde_json::Value`, and yields items
deserialized from `results`. Adapt a typed resource method by serializing
its `data` page:

```rust
use nomba_rs::pagination::paginate;
use futures::StreamExt;
use std::collections::HashMap;

let mut stream = paginate(
    |limit, cursor| {
        let accounts = nomba.virtual_accounts.clone();
        async move {
            accounts.filter_virtual_accounts(
                limit.map(|n| n.to_string()),
                cursor,
                None, None, None, None, None, None, None, None,
            )
            .await
            .and_then(|resp| serde_json::to_value(resp.data).map_err(nomba_rs::NombaError::from))
        }
    },
    Some(50),
);

while let Some(account) = stream.next().await {
    let account: HashMap<String, serde_json::Value> = account?;
    println!("{:?}", account.get("accountRef"));
}
```

## Concurrency Control

```rust
use nomba_rs::concurrency::gather_limited;

// Run up to 5 requests concurrently
let calls: Vec<_> = account_refs.iter().map(|account_ref| {
    let nomba = nomba.clone();
    let account_ref = account_ref.clone();
    move || async move {
        nomba.virtual_accounts.fetch_virtual_account(account_ref).await
    }
}).collect();

let results = gather_limited(calls, 5).await?;
```

## Error Handling

```rust
use nomba_rs::{NombaError, Result};

match nomba.virtual_accounts.create_virtual_account("ref", "Name", None, None, None, None, None).await {
    Ok(response) => println!("Success: {:?}", response),
    Err(NombaError::Api { status_code, code, .. }) => {
        eprintln!("API error: {} (status: {:?})", code.unwrap_or_default(), status_code);
    }
    Err(NombaError::Auth { .. }) => {
        eprintln!("Authentication failed");
    }
    Err(NombaError::Validation { missing, .. }) => {
        eprintln!("Validation failed: missing fields: {:?}", missing);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Local Request Validation

Enable the `validation` feature to validate requests locally before sending:

```toml
nomba-rs = { version = "0.5", features = ["validation"] }
```

```rust
use nomba_rs::validate_body;
use serde_json::json;

let body = json!({
    "accountRef": "ref-123",
    "accountName": "Test Account"
});

validate_body("post", "/v1/accounts/virtual", &body)?;
// Returns error if required fields are missing
```

## License

AGPL-3.0-only — Copyright (c) 2026 Righteousness Ude. See [LICENSE](LICENSE) for details.

Full guides and API reference: <https://RightFix.github.io/nomba-docs-rust/>

## Disclaimer

This is an unofficial SDK. Not affiliated with or endorsed by Nomba.
