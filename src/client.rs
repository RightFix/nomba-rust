use crate::error::Result;
use crate::http_client::{BlockingNombaClient, NombaClient, NombaClientConfig};
use crate::resources::{Auth, AsyncAuth, *};

pub struct Nomba {
    client: BlockingNombaClient,
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
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Self> {
        let config = NombaClientConfig::new(
            client_id.into(),
            client_secret.into(),
            account_id.into(),
        );
        let client = BlockingNombaClient::new(config)?;
        Self::from_client(client)
    }

    pub fn new_sandbox(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Self> {
        let config = NombaClientConfig::new(
            client_id.into(),
            client_secret.into(),
            account_id.into(),
        )
        .sandbox(true);
        let client = BlockingNombaClient::new(config)?;
        Self::from_client(client)
    }

    pub fn with_config(config: NombaClientConfig) -> Result<Self> {
        let client = BlockingNombaClient::new(config)?;
        Self::from_client(client)
    }

    fn from_client(client: BlockingNombaClient) -> Result<Self> {
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
            client,
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

    pub fn card_payment(&self, order_reference: impl Into<String>) -> crate::flows::CardPaymentFlow {
        crate::flows::CardPaymentFlow::new(self.charge.clone(), order_reference.into())
    }
}

pub struct AsyncNomba {
    client: NombaClient,
    pub accounts: AsyncAccounts,
    pub virtual_accounts: AsyncVirtualAccounts,
    pub checkout: AsyncCheckout,
    pub charge: AsyncCharge,
    pub transfers: AsyncTransfers,
    pub terminals: AsyncTerminals,
    pub transactions: AsyncTransactions,
    pub airtime_data: AsyncAirtimeData,
    pub cabletv: AsyncCableTv,
    pub electricity: AsyncElectricity,
    pub betting: AsyncBetting,
    pub direct_debits: AsyncDirectDebits,
    pub global_collections: AsyncGlobalCollections,
    pub global_payout: AsyncGlobalPayout,
    pub auth: AsyncAuth,
}

impl AsyncNomba {
    pub async fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Self> {
        let config = NombaClientConfig::new(
            client_id.into(),
            client_secret.into(),
            account_id.into(),
        );
        let client = NombaClient::new(config)?;
        Self::from_client(client).await
    }

    pub async fn new_sandbox(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Result<Self> {
        let config = NombaClientConfig::new(
            client_id.into(),
            client_secret.into(),
            account_id.into(),
        )
        .sandbox(true);
        let client = NombaClient::new(config)?;
        Self::from_client(client).await
    }

    pub async fn with_config(config: NombaClientConfig) -> Result<Self> {
        let client = NombaClient::new(config)?;
        Self::from_client(client).await
    }

    async fn from_client(client: NombaClient) -> Result<Self> {
        let accounts = AsyncAccounts::new(client.clone());
        let virtual_accounts = AsyncVirtualAccounts::new(client.clone());
        let checkout = AsyncCheckout::new(client.clone());
        let charge = AsyncCharge::new(client.clone());
        let transfers = AsyncTransfers::new(client.clone());
        let terminals = AsyncTerminals::new(client.clone());
        let transactions = AsyncTransactions::new(client.clone());
        let airtime_data = AsyncAirtimeData::new(client.clone());
        let cabletv = AsyncCableTv::new(client.clone());
        let electricity = AsyncElectricity::new(client.clone());
        let betting = AsyncBetting::new(client.clone());
        let direct_debits = AsyncDirectDebits::new(client.clone());
        let global_collections = AsyncGlobalCollections::new(client.clone());
        let global_payout = AsyncGlobalPayout::new(client.clone());
        let auth = AsyncAuth::new(client.clone());

        Ok(Self {
            client,
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
    ) -> crate::flows::AsyncCardPaymentFlow {
        crate::flows::AsyncCardPaymentFlow::new(self.charge.clone(), order_reference.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config() {
        let config = NombaClientConfig::new("test_id".into(), "test_secret".into(), "test_account".into())
            .sandbox(true)
            .timeout(std::time::Duration::from_secs(60));
        assert!(config.sandbox);
        assert_eq!(config.timeout, std::time::Duration::from_secs(60));
    }
}