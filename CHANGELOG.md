# Changelog

All notable changes to `nomba-rs` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2026-10-06

First crates.io release as `nomba-rs` (author: Righteousness Ude,
license: AGPL-3.0-only). Endpoints aligned with the current
[Nomba API documentation](https://developer.nomba.com) — deprecated routes
removed, missing current routes added.

### Added

- Crate-level docs pointing at <https://RightFix.github.io/nomba-docs-rust/>.
- Auth: `refresh_access_token` (`POST /v1/auth/token/refresh`) on `Auth` /
  `AsyncAuth` with new `RefreshTokenResponse` model.
- Checkout: `charge_with_tokenized_card`, `list_tokenized_cards`,
  `update_tokenized_card`, `delete_tokenized_card` (canonical home per the
  Online Checkout docs); `request_saved_cards_otp`
  (`POST /v1/checkout/user-card/saved-card/auth`).
- Global Payout: `convert_money`, `fetch_exchange_rates(from, to, region)`,
  `fetch_payment_methods`, `list_institution_providers`,
  `fetch_accounts_sandbox`, `fetch_account_sandbox` with new
  `PaymentMethodData` / `InstitutionProviderData` models.
- Global Collections: `fetch_drc_inflow_providers_sandbox`
  (`GET /v1/sandbox/global-collection/drc/inflow/providers`).
- Transfers: `fetch_bank_codes` (`GET /v1/transfers/banks`) and
  `bank_account_lookup` (`POST /v1/transfers/bank/lookup`).

### Changed

- Crate renamed `nomba` -> `nomba-rs` (import as `nomba_rs`).
- Bills vending moved to the current `POST /v2/bill/*` routes (airtime topup,
  data, electricity, cable TV, betting, each parent + sub-account).
- Checkout user-card flows corrected to the live contracts:
  `fetch_transaction_details` now `POST /v1/checkout/confirm-transaction-receipt`;
  `request_user_otp` takes `phone_number`; `submit_user_otp` posts to
  `POST /v1/checkout/user-card` with `phone_number`; `fetch_user_saved_cards`
  takes `otp` as a query parameter.
- Global Collections `initiate_mobile_money_inflow` now takes
  `(phone_number, callback_url, amount, currency, topup_vendor, idempotency_key)`
  per the live contract; `fetch_drc_inflow_providers` no longer takes a
  `sandbox` flag (use `fetch_drc_inflow_providers_sandbox`).
- Terminals `send_payment_request` now takes `(terminal_id, merchant_tx_ref,
  amount, currency)` and sends `merchantTxRef` per the live contract.
- Fixed a token-cache deadlock in the async client (`ensure_token` no longer
  holds the mutex across the token fetch; 401 invalidation is now async).
- `LICENSE` replaced with the canonical AGPL-3.0 text.

### Removed

- Deprecated `POST /v1/bill/*` vending routes (superseded by `/v2/bill/*`).
- `Transfers::{authorize_transfer, authorize_exchange, convert_money,
  fetch_exchange_rates}` — incorrect duplicates of Global Payout operations
  with non-standard signatures/bodies; use `GlobalPayout::*` instead.
- `Charge::{fetch_bank_codes, bank_account_lookup}` — moved to `Transfers`.
- `Transfers` sub-account bank/wallet helpers unchanged; no other removals.

### Packaging

- `Cargo.toml`: `authors`, `homepage`/`documentation` (docs site), corrected
  `repository` (`RightFix/nomba-rust`), SPDX `AGPL-3.0-only`, lighter `async`
  feature (no longer pulls `tokio/full`).
- `cargo publish --dry-run` passes; `cargo test --all-features`,
  `cargo fmt --check` clean.
