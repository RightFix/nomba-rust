use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct Accounts {
    client: BlockingNombaClient,
}

impl Accounts {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn list_all_accounts(&self) -> Result<NombaResponse<PaginatedResponse<SubAccountData>>> {
        let response = self.client.get("/v1/accounts", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn create_sub_account(
        &self,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        phone_number: impl Into<String>,
        email: impl Into<String>,
        bvn: impl Into<String>,
        account_type: impl Into<String>,
        currency: impl Into<String>,
        callback_url: impl Into<String>,
    ) -> Result<CreateSubAccountResponse> {
        let body = json!({
            "accountRef": account_ref.into(),
            "accountName": account_name.into(),
            "phoneNumber": phone_number.into(),
            "email": email.into(),
            "bvn": bvn.into(),
            "type": account_type.into(),
            "currency": currency.into(),
            "callbackUrl": callback_url.into(),
        });

        let response = self.client.post("/v1/accounts", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_account_details(
        &self,
        account_id: impl Into<String>,
    ) -> Result<FetchAccountDetailsResponse> {
        let path = format!("/v1/accounts/{}", account_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_parent_account_details(&self) -> Result<FetchAccountDetailsResponse> {
        let response = self.client.get("/v1/accounts/parent", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_account_balance(
        &self,
        account_id: impl Into<String>,
    ) -> Result<FetchAccountBalanceResponse> {
        let path = format!("/v1/accounts/{}/balance", account_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_parent_account_balance(&self) -> Result<FetchAccountBalanceResponse> {
        let response = self.client.get("/v1/accounts/parent/balance", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn suspend_account(&self, account_id: impl Into<String>) -> Result<SuspendAccountResponse> {
        let path = format!("/v1/accounts/{}/suspend", account_id.into());
        let response = self.client.post(&path, &json!({}), None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn reactivate_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<ReactivateAccountResponse> {
        let path = format!("/v1/accounts/{}/reactivate", account_id.into());
        let response = self.client.post(&path, &json!({}), None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_terminals_assigned_to_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<NombaResponse<PaginatedResponse<TerminalAssignmentData>>> {
        let path = format!("/v1/accounts/{}/terminals", account_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_terminals_assigned_to_parent_account(
        &self,
    ) -> Result<NombaResponse<PaginatedResponse<TerminalAssignmentData>>> {
        let response = self.client.get("/v1/accounts/parent/terminals", None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_terminals_assigned_to_sub_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<NombaResponse<PaginatedResponse<TerminalAssignmentData>>> {
        let path = format!("/v1/accounts/{}/terminals/sub", account_id.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn update_account_access(
        &self,
        account_id: impl Into<String>,
        terminal_id: impl Into<String>,
        access: bool,
    ) -> Result<UpdateAccountAccessResponse> {
        let path = format!("/v1/accounts/{}/terminals/{}/access", account_id.into(), terminal_id.into());
        let body = json!({ "access": access });
        let response = self.client.put(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncAccounts {
    client: NombaClient,
}

impl AsyncAccounts {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn list_all_accounts(&self) -> Result<NombaResponse<PaginatedResponse<SubAccountData>>> {
        let response = self.client.get("/v1/accounts", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create_sub_account(
        &self,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        phone_number: impl Into<String>,
        email: impl Into<String>,
        bvn: impl Into<String>,
        account_type: impl Into<String>,
        currency: impl Into<String>,
        callback_url: impl Into<String>,
    ) -> Result<CreateSubAccountResponse> {
        let body = json!({
            "accountRef": account_ref.into(),
            "accountName": account_name.into(),
            "phoneNumber": phone_number.into(),
            "email": email.into(),
            "bvn": bvn.into(),
            "type": account_type.into(),
            "currency": currency.into(),
            "callbackUrl": callback_url.into(),
        });

        let response = self.client.post("/v1/accounts", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_account_details(
        &self,
        account_id: impl Into<String>,
    ) -> Result<FetchAccountDetailsResponse> {
        let path = format!("/v1/accounts/{}", account_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_parent_account_details(&self) -> Result<FetchAccountDetailsResponse> {
        let response = self.client.get("/v1/accounts/parent", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_account_balance(
        &self,
        account_id: impl Into<String>,
    ) -> Result<FetchAccountBalanceResponse> {
        let path = format!("/v1/accounts/{}/balance", account_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_parent_account_balance(&self) -> Result<FetchAccountBalanceResponse> {
        let response = self.client.get("/v1/accounts/parent/balance", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn suspend_account(&self, account_id: impl Into<String>) -> Result<SuspendAccountResponse> {
        let path = format!("/v1/accounts/{}/suspend", account_id.into());
        let response = self.client.post(&path, &json!({}), None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn reactivate_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<ReactivateAccountResponse> {
        let path = format!("/v1/accounts/{}/reactivate", account_id.into());
        let response = self.client.post(&path, &json!({}), None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_terminals_assigned_to_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<NombaResponse<PaginatedResponse<TerminalAssignmentData>>> {
        let path = format!("/v1/accounts/{}/terminals", account_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_terminals_assigned_to_parent_account(
        &self,
    ) -> Result<NombaResponse<PaginatedResponse<TerminalAssignmentData>>> {
        let response = self.client.get("/v1/accounts/parent/terminals", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_terminals_assigned_to_sub_account(
        &self,
        account_id: impl Into<String>,
    ) -> Result<NombaResponse<PaginatedResponse<TerminalAssignmentData>>> {
        let path = format!("/v1/accounts/{}/terminals/sub", account_id.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update_account_access(
        &self,
        account_id: impl Into<String>,
        terminal_id: impl Into<String>,
        access: bool,
    ) -> Result<UpdateAccountAccessResponse> {
        let path = format!("/v1/accounts/{}/terminals/{}/access", account_id.into(), terminal_id.into());
        let body = json!({ "access": access });
        let response = self.client.put(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}