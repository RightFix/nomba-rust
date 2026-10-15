use crate::error::Result;
use crate::http_client::{NombaClient, NombaClientConfig};
use crate::resources::*;

/// Async high-level entry point for the Nomba SDK.
///
/// Holds one resource handle per API group (`accounts`, `virtual_accounts`,
/// `checkout`, …); all handles share the same HTTP connection pool and
/// OAuth2 token cache. Every resource method is `async` and must be
/// `.await`ed.
#[derive(Clone)]
pub struct Nomba {
    pub accounts: Accounts,
    pub virtual_accounts: VirtualAccounts,
    pub checkout: Checkout,
    pub charge: Charge,
    pub transfers: Transfers,
    pub terminals: Terminals,
    pub transactions: Transactions,
    pub airtime_data: AirtimeData,
    pub cabletv: CableTv,
    pub electricity: Electricity,
    pub betting: Betting,
    pub direct_debits: DirectDebits,
    pub global_collections: GlobalCollections,
    pub global_payout: GlobalPayout,
    pub auth: Auth,
}

impl Nomba {
    pub async fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Self> {
        let config =
            NombaClientConfig::new(client_id.into(), client_secret.into(), account_id.into());
        let client = NombaClient::new(config)?;
        Self::from_client(client).await
    }

    pub async fn new_sandbox(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Self> {
        let config =
            NombaClientConfig::new(client_id.into(), client_secret.into(), account_id.into())
                .sandbox(true);
        let client = NombaClient::new(config)?;
        Self::from_client(client).await
    }

    pub async fn with_config(config: NombaClientConfig) -> Result<Self> {
        let client = NombaClient::new(config)?;
        Self::from_client(client).await
    }

    async fn from_client(client: NombaClient) -> Result<Self> {
        let accounts = Accounts::new(client.clone());
        let virtual_accounts = VirtualAccounts::new(client.clone());
        let checkout = Checkout::new(client.clone());
        let charge = Charge::new(client.clone());
        let transfers = Transfers::new(client.clone());
        let terminals = Terminals::new(client.clone());
        let transactions = Transactions::new(client.clone());
        let airtime_data = AirtimeData::new(client.clone());
        let cabletv = CableTv::new(client.clone());
        let electricity = Electricity::new(client.clone());
        let betting = Betting::new(client.clone());
        let direct_debits = DirectDebits::new(client.clone());
        let global_collections = GlobalCollections::new(client.clone());
        let global_payout = GlobalPayout::new(client.clone());
        let auth = Auth::new(client.clone());

        Ok(Self {
            accounts,
            virtual_accounts,
            checkout,
            charge,
            transfers,
            terminals,
            transactions,
            airtime_data,
            cabletv,
            electricity,
            betting,
            direct_debits,
            global_collections,
            global_payout,
            auth,
        })
    }

    pub fn card_payment(
        &self,
        order_reference: impl Into<String>,
    ) -> crate::flows::CardPaymentFlow {
        crate::flows::CardPaymentFlow::new(self.charge.clone(), order_reference.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config() {
        let config = NombaClientConfig::new(
            "test_id".into(),
            "test_secret".into(),
            "test_account".into(),
        )
        .sandbox(true)
        .timeout(std::time::Duration::from_secs(60));
        assert!(config.sandbox);
        assert_eq!(config.timeout, std::time::Duration::from_secs(60));
    }
}
