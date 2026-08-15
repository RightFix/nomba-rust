use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Transactions {
    client: BlockingNombaClient,
}

impl Transactions {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn fetch_credit_debit_on_sub_account(
        &self,
        account_id: impl Into<String>,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let path = format!("/v1/transactions/credit-debit/{}", account_id.into());
        let response = self.client.get(&path, Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_credit_debit_on_parent(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let response = self.client.get("/v1/transactions/credit-debit/parent", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_on_sub_account(
        &self,
        account_id: impl Into<String>,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let path = format!("/v1/transactions/{}", account_id.into());
        let response = self.client.get(&path, Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn filter_sub_account_transactions(
        &self,
        account_id: impl Into<String>,
        limit: Option<String>,
        cursor: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        status: Option<String>,
        transaction_type: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        if let Some(from_date) = from_date {
            params.push(("fromDate", from_date));
        }
        if let Some(to_date) = to_date {
            params.push(("toDate", to_date));
        }
        if let Some(status) = status {
            params.push(("status", status));
        }
        if let Some(transaction_type) = transaction_type {
            params.push(("type", transaction_type));
        }
        let path = format!("/v1/transactions/filter/{}", account_id.into());
        let response = self.client.get(&path, Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_on_parent(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let response = self.client.get("/v1/transactions/parent", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn filter_parent_account_transactions(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        status: Option<String>,
        transaction_type: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        if let Some(from_date) = from_date {
            params.push(("fromDate", from_date));
        }
        if let Some(to_date) = to_date {
            params.push(("toDate", to_date));
        }
        if let Some(status) = status {
            params.push(("status", status));
        }
        if let Some(transaction_type) = transaction_type {
            params.push(("type", transaction_type));
        }
        let response = self.client.get("/v1/transactions/filter/parent", Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_single_on_sub_account(
        &self,
        account_id: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Result<FetchTransactionResponse> {
        let path = format!("/v1/transactions/{}/{}", account_id.into(), transaction_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_single_on_parent(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchTransactionResponse> {
        let path = format!("/v1/transactions/parent/{}", transaction_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn confirm_by_session_id(
        &self,
        session_id: impl Into<String>,
    ) -> Result<ConfirmTransactionBySessionResponse> {
        let path = format!("/v1/transactions/session/{}", session_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncTransactions {
    client: NombaClient,
}

impl AsyncTransactions {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn fetch_credit_debit_on_sub_account(
        &self,
        account_id: impl Into<String>,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let path = format!("/v1/transactions/credit-debit/{}", account_id.into());
        let response = self.client.get(&path, Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_credit_debit_on_parent(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let response = self.client.get("/v1/transactions/credit-debit/parent", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_on_sub_account(
        &self,
        account_id: impl Into<String>,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let path = format!("/v1/transactions/{}", account_id.into());
        let response = self.client.get(&path, Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn filter_sub_account_transactions(
        &self,
        account_id: impl Into<String>,
        limit: Option<String>,
        cursor: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        status: Option<String>,
        transaction_type: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        if let Some(from_date) = from_date {
            params.push(("fromDate", from_date));
        }
        if let Some(to_date) = to_date {
            params.push(("toDate", to_date));
        }
        if let Some(status) = status {
            params.push(("status", status));
        }
        if let Some(transaction_type) = transaction_type {
            params.push(("type", transaction_type));
        }
        let path = format!("/v1/transactions/filter/{}", account_id.into());
        let response = self.client.get(&path, Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_on_parent(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        let response = self.client.get("/v1/transactions/parent", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn filter_parent_account_transactions(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        status: Option<String>,
        transaction_type: Option<String>,
    ) -> Result<FilterTransactionsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }
        if let Some(from_date) = from_date {
            params.push(("fromDate", from_date));
        }
        if let Some(to_date) = to_date {
            params.push(("toDate", to_date));
        }
        if let Some(status) = status {
            params.push(("status", status));
        }
        if let Some(transaction_type) = transaction_type {
            params.push(("type", transaction_type));
        }
        let response = self.client.get("/v1/transactions/filter/parent", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_single_on_sub_account(
        &self,
        account_id: impl Into<String>,
        transaction_id: impl Into<String>,
    ) -> Result<FetchTransactionResponse> {
        let path = format!("/v1/transactions/{}/{}", account_id.into(), transaction_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_single_on_parent(
        &self,
        transaction_id: impl Into<String>,
    ) -> Result<FetchTransactionResponse> {
        let path = format!("/v1/transactions/parent/{}", transaction_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn confirm_by_session_id(
        &self,
        session_id: impl Into<String>,
    ) -> Result<ConfirmTransactionBySessionResponse> {
        let path = format!("/v1/transactions/session/{}", session_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}