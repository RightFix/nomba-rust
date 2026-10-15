# Changelog

All notable changes to `nomba-rs` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.5.1] - 2026-10-15

### Removed

- **Breaking:** the entire synchronous (blocking) API is gone — the crate is
  now async-only on Tokio. Removed `Nomba`'s blocking twin set: all `Async*`
  prefixes are dropped and the async types take the base names (`Nomba`,
  `Accounts`, `Checkout`, `CardPaymentFlow`, `paginate`, `Paginator`, …).
  The `blocking` / `async` Cargo features are removed; only `validation`
  remains. Migration: add `.await` to every call and run under Tokio.

### Added

- Optional `idempotency_key` on `authorize_transfer` and
  `authorize_exchange`; responses expose echoed `idempotency_key` and
  `idempotent_replay`.
- Optional `restrict_inflow_to_account_name` on virtual-account creation.
- Typed bill/lookup shapes from the live API docs: `DataPlan`,
  `CustomerLookupData` (string-or-map with `name()`), `ElectricityCustomerData`
  (with `address`), `BankData` (`code`/`name`/`nipCode`/`logo`),
  `BankAccountLookupData`, `InflowProviderData`, `ExchangeRateData`.
- New response fields: bill data `id`/`source`/`customerBillerId`/`productId`
  cohorts, collection `vendor`/`gatewayMessage`/`gatewayCode`, payout
  `idempotencyKey`/`idempotentReplay`, virtual-account
  `restrictInflowToAccountName`.

### Changed

- `Electricity::customer_lookup` moved to `GET /v2/bill/electricity/lookup`
  (v1 lookup deprecated) with `(disco, customer_id)` parameters.
- `update_virtual_account` no longer takes `callback_url` (dropped by the API).
- Vending `fee` is now optional (absent on v2 responses).
- New `tests/fixtures/` live-shape conformance suite: documentation example
  payloads asserted against the response structs.

## [0.5.0] - 2026-10-13

### Added

- Optional `payer_name` on cable TV subscriptions: all four
  `subscribe_parent` / `subscribe_account` constructors (blocking and
  async) take a new `payer_name: Option<String>` parameter, sent as the
  `payerName` body field when `Some` (use the account owner's name).

### Changed

- **Breaking:** the four cable subscribe constructors take one more
  argument (`payer_name` after `phone_number`).

## [0.4.2] - 2026-10-12

### Fixed

- Sandbox checkout orders now send the nested `{"order": ...}` body
  (flat bodies fail sandbox validation with `[order: order can not be
  null]`); live keeps the flat shape. Shared `checkout_order_body`
  helper with unit tests for both shapes.

## [0.4.1] - 2026-10-12

### Fixed

- `VirtualAccountData.callback_url` is now `Option<String>`: the API
  omits it on some accounts (verified against a live NIN-created sandbox
  account), which failed deserialization of every such response.

## [0.4.0] - 2026-10-11

### Added

- `nin` support on virtual-account creation: all four
  `create_virtual_account[_for_sub_account]` constructors (blocking and
  async) take a new `nin: Option<String>` parameter, sent as the
  documented 11-digit `nin` body field on its own or alongside `bvn`.
  Shared `build_virtual_account_body` helper plus unit tests.
- `VirtualAccountData` now exposes the identifier the account was
  created against: `nin` and `identifierType` (`BVN`/`NIN`/`BVN_AND_NIN`)
  as `Option`s, and `bvn` relaxed to `Option` (absent on NIN-only
  accounts — previously a required `String`, which failed
  deserialization of NIN-created accounts).
- Bundled OpenAPI spec gains the `nin` property on
  `CreateVirtualAccountRequest`; README shows the NIN call shape.

### Changed

- **Breaking:** the four virtual-account constructors take one more
  argument (`nin` after `bvn`); `VirtualAccountData.bvn` is now
  `Option<String>`.

## [0.3.0] - 2026-10-10

### Fixed

- Plan/provider list responses no longer fail deserialization when Nomba
  returns `data` as an array: `FetchDataPlansResponse.data` and
  `FetchBettingProvidersResponse.data` are now the new `PlanListData`
  untagged enum (`List(Vec<Value>)` / `Map(HashMap<String, Value>)`,
  with an `as_list()` helper) instead of a bare `HashMap`, and
  `FetchElectricityProvidersResponse.data` is now
  `Vec<ElectricityProviderData>` (the discos endpoint returns a list of
  `{id, name}`, not one object). Verified against live sandbox
  responses for data plans, betting providers, and discos.
- **Breaking:** the three `data` field types above changed; match on
  `PlanListData` (or serialize the response back to `Value`) instead of
  using them as maps/structs directly.

## [0.2.0] - 2026-10-06

### Changed

- **Breaking:** `gather_limited` and `gather_limited_ordered` no longer take
  the `return_exceptions` flag — it never had any effect (both paths returned
  the first error). Both now take `(calls, limit)`.
- `AsyncPaginator` no longer requires `Unpin` futures/closures, so natural
  `async move` blocks work. It also keeps the in-flight request across polls
  instead of restarting it on every wakeup.

### Fixed

- Removed the unused `serde_with` dependency.
- Bundled OpenAPI spec extended with the current `POST /v2/bill/*` routes and
  sandbox Global Payout paths, so `validate_body` now checks them instead of
  silently passing.
- `paginate`/`apaginate` docs and README show a working adapter from typed
  resource methods to the raw-page closure (with compile-checked examples).
- Clippy-clean: `too_many_arguments` exempted crate-wide (methods mirror the
  API), plus `vec!`, `next_back`, `strip_suffix`, `is_some_and` cleanups and
  removed dead `Nomba`/`AsyncNomba` client fields.
- Documented that the blocking client must not be used from inside async code
  (use `AsyncNomba` there).

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
