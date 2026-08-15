# Nomba Rust SDK

Unofficial Rust SDK for the [Nomba](https://developer.nomba.com) payments API.

## Features

- **Sync & Async clients** - Both blocking and async/await APIs
- **Complete API coverage** - All Nomba endpoints (Accounts, Virtual Accounts, Checkout, Transfers, Terminals, Transactions, Airtime/Data, CableTV, Electricity, Betting, Direct Debits, Global Collections, Global Payout)
- **OAuth2 authentication** - Automatic token management with refresh
- **Retry logic** - Automatic retry with exponential backoff for 429/5xx errors
- **Webhook verification** - HMAC-SHA256 signature verification with replay protection
- **Card payment flows** - Guided multi-step card payment (OTP, 3DS)
- **Pagination helpers** - Cursor-based pagination iterators
- **Bounded concurrency** - Rate-limited concurrent request execution
- **Request validation** - Optional local validation against OpenAPI spec

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nomba = { git = "https://github.com/RightFix/nomba", branch = "main" }
```

Or for local development:
```toml
[dependencies]
nomba = { path = "../nomba-rust" }
```

### Features

- `blocking` (default) - Enable sync/blocking client
- `async` (default) - Enable async client (requires tokio)
- `validation` - Enable local request validation against OpenAPI spec

```toml
nomba = { version = "0.1", features = ["async", "validation"] }
```

## Quick Start

### Sync Client

```rust
use nomba::{Nomba, NombaClientConfig};

fn main() -> nomba::Result<()> {
    let nomba = Nomba::new(
        "your_client_id",
        "your_client_secret",
        "your_account_id",
    )?;

    // Create a virtual account
    let account = nomba.virtual_accounts.create_virtual_account(
        "ref-123",
        "Jane Doe",
        None, None, None
    )?;
    
    println!("Created account: {:?}", account.data.account_ref);
    Ok(())
}
```

### Async Client

```rust
use nomba::AsyncNomba;

#[tokio::main]
async fn main() -> nomba::Result<()> {
    let nomba = AsyncNomba::new(
        "your_client_id",
        "your_client_secret",
        "your_account_id",
    ).await?;

    // Create a virtual account
    let account = nomba.virtual_accounts.create_virtual_account(
        "ref-123",
        "Jane Doe",
        None, None, None
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
)?;
```

### Configuration

```rust
use nomba::NombaClientConfig;
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

let nomba = Nomba::with_config(config)?;
```

## Resource Modules

Each API group is exposed as a resource on the client:

| Resource | Sync | Async |
|----------|------|-------|
| Accounts | `nomba.accounts` | `nomba.accounts` |
| Virtual Accounts | `nomba.virtual_accounts` | `nomba.virtual_accounts` |
| Checkout | `nomba.checkout` | `nomba.checkout` |
| Charge | `nomba.charge` | `nomba.charge` |
| Transfers | `nomba.transfers` | `nomba.transfers` |
| Terminals | `nomba.terminals` | `nomba.terminals` |
| Transactions | `nomba.transactions` | `nomba.transactions` |
| Airtime & Data | `nomba.airtime_data` | `nomba.airtime_data` |
| CableTV | `nomba.cabletv` | `nomba.cabletv` |
| Electricity | `nomba.electricity` | `nomba.electricity` |
| Betting | `nomba.betting` | `nomba.betting` |
| Direct Debits | `nomba.direct_debits` | `nomba.direct_debits` |
| Global Collections | `nomba.global_collections` | `nomba.global_collections` |
| Global Payout | `nomba.global_payout` | `nomba.global_payout` |
| Auth | `nomba.auth` | `nomba.auth` |

## Auth - Revoke Access Token

```rust
use nomba::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id")?;

// Revoke an access token (e.g., on logout)
let revoked = nomba.auth.revoke_access_token("access_token_to_revoke")?;
println!("Token revoked: {}", revoked.description);
```

## Global Payout

```rust
use nomba::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id")?;

// Fetch all currency wallets
let accounts = nomba.global_payout.fetch_accounts()?;
for wallet in accounts.data {
    println!("Wallet: {} ({}) - Balance: {:?}", wallet.name, wallet.currency, wallet.balance);
}

// Fetch a specific wallet by accountId (not currency code!)
let wallet = nomba.global_payout.fetch_account("66bc8c0e054dfe06b69a840a")?;
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
)?;
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
)?;
println!("Exchange: {}", exchange.data.wt_transaction_id);
```

## CableTV - Fetch Plans

```rust
use nomba::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id")?;

// Get available DStv plans
let plans = nomba.cabletv.fetch_plans("dstv")?;
for plan in plans.data {
    println!("Plan: {} - ₦{}", plan.subscription_type, plan.amount);
}

// Also supports: gotv, startimes, ShowMax
```

## Airtime & Data - Data Vending with Product ID

```rust
use nomba::Nomba;

let nomba = Nomba::new("client_id", "client_secret", "account_id")?;

// 1. Fetch data plans to get product IDs
let plans = nomba.airtime_data.fetch_data_plans("MTN")?;

// 2. Vend data using product_id (not amount!)
let vend = nomba.airtime_data.vend_data_parent(
    "mtn47",                        // product_id from plans response
    "08055441122",                  // phone_number
    "MTN",                          // network
    "txn-ref-123",                  // merchant_tx_ref
    Some("John Doe"),               // sender_name
)?;
println!("Data vended: {}", vend.data.amount);
```

## Card Payment Flow

The SDK provides a guided flow for card payments:

```rust
use nomba::Nomba;

let nomba = Nomba::new(...)?;

// 1. Create checkout order
let order = nomba.checkout.create_order(
    "order-123",
    "1000",
    "NGN",
    "customer@example.com",
    "John Doe",
    "https://example.com/redirect",
    None, None
)?;

let order_ref = order.data.order_reference;

// 2. Start card payment flow
let mut flow = nomba.card_payment(order_ref);

// 3. Submit card details
let step = flow.submit_card(
    "encrypted_card_details",
    "rsa_public_key",
    Some(true),
    None
)?;

if step.requires_otp {
    // 4. Submit OTP
    let step = flow.submit_otp("123456")?;
}

if step.requires_3ds {
    // Redirect user using step.secure_authentication_data
}

// 5. Confirm final status
if step.completed {
    let result = flow.confirm()?;
    println!("Payment successful: {:?}", result);
}
```

## Webhook Verification

```rust
use nomba::webhooks::{verify_webhook_request, check_timestamp_freshness};
use std::collections::HashMap;

fn handle_webhook(body: &[u8], headers: HashMap<String, String>) -> nomba::Result<()> {
    let payload = verify_webhook_request(
        "your_webhook_signature_key",
        body,
        &headers,
        Some(300.0), // 5 min replay protection
    )?;
    
    // Process webhook payload
    println!("Received: {:?}", payload);
    Ok(())
}
```

## Pagination

```rust
use nomba::pagination::paginate;

// Sync pagination
for account in paginate(|limit, cursor| {
    nomba.virtual_accounts.filter_virtual_accounts(limit, cursor, None, None, None, None, None, None, None, None)
}, Some(50)) {
    println!("Account: {:?}", account?.account_ref);
}

// Async pagination
use nomba::pagination::apaginate;
use futures::StreamExt;

let mut stream = apaginate(|limit, cursor| {
    nomba.virtual_accounts.filter_virtual_accounts(limit, cursor, None, None, None, None, None, None, None, None)
}, Some(50));

while let Some(account) = stream.next().await {
    println!("Account: {:?}", account?.account_ref);
}
```

## Concurrency Control

```rust
use nomba::concurrency::gather_limited;

// Run up to 5 requests concurrently
let calls: Vec<_> = account_refs.iter().map(|ref| {
    let nomba = nomba.clone();
    let ref = ref.clone();
    move || async move {
        nomba.virtual_accounts.fetch_virtual_account(ref)
    }
}).collect();

let results = gather_limited(calls, 5, false).await?;
```

## Error Handling

```rust
use nomba::{NombaError, Result};

match nomba.virtual_accounts.create_virtual_account(...) {
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
nomba = { version = "0.1", features = ["validation"] }
```

```rust
use nomba::validate_body;
use serde_json::json;

let body = json!({
    "accountRef": "ref-123",
    "accountName": "Test Account"
});

validate_body("post", "/v1/accounts/virtual", &body)?;
// Returns error if required fields are missing
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Disclaimer

This is an unofficial SDK. Not affiliated with or endorsed by Nomba.