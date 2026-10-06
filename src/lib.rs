//! `nomba-rs` — unofficial Rust SDK for the [Nomba payments API](https://developer.nomba.com).
//!
//! Full guides and per-endpoint reference live at
//! <https://RightFix.github.io/nomba-docs-rust/>.
//!
//! ```no_run
//! use nomba_rs::Nomba;
//!
//! let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
//! let account = nomba.virtual_accounts.create_virtual_account(
//!     "ref-123",
//!     "Jane Doe",
//!     None, None, None
//! )?;
//! # Ok::<(), nomba_rs::NombaError>(())
//! ```

#[cfg(feature = "validation")]
mod validation;

pub mod client;
pub mod concurrency;
pub mod error;
pub mod flows;
pub mod http_client;
pub mod models;
pub mod pagination;
pub mod resources;
pub mod webhooks;

pub use client::{AsyncNomba, Nomba};
pub use concurrency::{gather_limited, gather_limited_ordered};
pub use error::{NombaError, Result};
pub use flows::{AsyncCardPaymentFlow, CardPaymentFlow, CardPaymentStep};
pub use http_client::{BlockingNombaClient, NombaClient, NombaClientConfig};
pub use models::*;
pub use pagination::{apaginate, paginate, AsyncPaginator, Paginator};
pub use resources::*;
pub use webhooks::{
    check_timestamp_freshness, compute_signature, verify_webhook_request, verify_webhook_signature,
};

#[cfg(feature = "validation")]
pub use validation::validate_body;
