use crate::http_client::BlockingNombaClient;
use crate::error::Result;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

#[derive(Clone)]
pub struct VirtualAccounts {
    client: BlockingNombaClient,
}

impl VirtualAccounts {
    pub fn new(client: BlockingNombaClient) -> Self {
        Self { client }
    }

    pub fn create_virtual_account(
        &self,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        bvn: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let mut body = json!({
            "accountRef": account_ref.into(),
            "accountName": account_name.into(),
        });

        if let Some(bvn) = bvn {
            body["bvn"] = json!(bvn);
        }
        if let Some(expiry_date) = expiry_date {
            body["expiryDate"] = json!(expiry_date);
        }
        if let Some(expected_amount) = expected_amount {
            body["expectedAmount"] = json!(expected_amount);
        }

        let response = self.client.post("/v1/accounts/virtual", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn create_virtual_account_for_sub_account(
        &self,
        sub_account_id: impl Into<String>,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        bvn: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let mut body = json!({
            "accountRef": account_ref.into(),
            "accountName": account_name.into(),
        });

        if let Some(bvn) = bvn {
            body["bvn"] = json!(bvn);
        }
        if let Some(expiry_date) = expiry_date {
            body["expiryDate"] = json!(expiry_date);
        }
        if let Some(expected_amount) = expected_amount {
            body["expectedAmount"] = json!(expected_amount);
        }

        let path = format!("/v1/accounts/virtual/{}", sub_account_id.into());
        let response = self.client.post(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn filter_virtual_accounts(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
        account_name: Option<String>,
        account_ref: Option<String>,
        bvn: Option<String>,
        bank_account_number: Option<String>,
        date_created_from: Option<String>,
        date_created_to: Option<String>,
        expired: Option<bool>,
        resource_acquired: Option<bool>,
    ) -> Result<FilterVirtualAccountsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }

        let mut body = json!({});
        if let Some(account_name) = account_name {
            body["accountName"] = json!(account_name);
        }
        if let Some(account_ref) = account_ref {
            body["accountRef"] = json!(account_ref);
        }
        if let Some(bvn) = bvn {
            body["bvn"] = json!(bvn);
        }
        if let Some(bank_account_number) = bank_account_number {
            body["bankAccountNumber"] = json!(bank_account_number);
        }
        if let Some(date_created_from) = date_created_from {
            body["dateCreatedFrom"] = json!(date_created_from);
        }
        if let Some(date_created_to) = date_created_to {
            body["dateCreatedTo"] = json!(date_created_to);
        }
        if let Some(expired) = expired {
            body["expired"] = json!(expired);
        }
        if let Some(resource_acquired) = resource_acquired {
            body["resourceAcquired"] = json!(resource_acquired);
        }

        let response = self.client.post("/v1/accounts/virtual/list", &body, Some(params))?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn update_virtual_account(
        &self,
        identifier: impl Into<String>,
        new_account_ref: Option<String>,
        account_name: Option<String>,
        callback_url: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<UpdateVirtualAccountResponse> {
        let mut body = json!({});
        if let Some(new_account_ref) = new_account_ref {
            body["newAccountRef"] = json!(new_account_ref);
        }
        if let Some(account_name) = account_name {
            body["accountName"] = json!(account_name);
        }
        if let Some(callback_url) = callback_url {
            body["callbackUrl"] = json!(callback_url);
        }
        if let Some(expected_amount) = expected_amount {
            body["expectedAmount"] = json!(expected_amount);
        }

        let path = format!("/v1/accounts/virtual/{}", identifier.into());
        let response = self.client.put(&path, &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn fetch_virtual_account(
        &self,
        identifier: impl Into<String>,
    ) -> Result<FetchVirtualAccountResponse> {
        let path = format!("/v1/accounts/virtual/{}", identifier.into());
        let response = self.client.get(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn expire_virtual_account(
        &self,
        identifier: impl Into<String>,
    ) -> Result<ExpireVirtualAccountResponse> {
        let path = format!("/v1/accounts/virtual/{}", identifier.into());
        let response = self.client.delete(&path, None)?;
        Ok(serde_json::from_value(response)?)
    }
}

#[derive(Clone)]
pub struct AsyncVirtualAccounts {
    client: NombaClient,
}

impl AsyncVirtualAccounts {
    pub fn new(client: NombaClient) -> Self {
        Self { client }
    }

    pub async fn create_virtual_account(
        &self,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        bvn: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let mut body = json!({
            "accountRef": account_ref.into(),
            "accountName": account_name.into(),
        });

        if let Some(bvn) = bvn {
            body["bvn"] = json!(bvn);
        }
        if let Some(expiry_date) = expiry_date {
            body["expiryDate"] = json!(expiry_date);
        }
        if let Some(expected_amount) = expected_amount {
            body["expectedAmount"] = json!(expected_amount);
        }

        let response = self.client.post("/v1/accounts/virtual", &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create_virtual_account_for_sub_account(
        &self,
        sub_account_id: impl Into<String>,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        bvn: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let mut body = json!({
            "accountRef": account_ref.into(),
            "accountName": account_name.into(),
        });

        if let Some(bvn) = bvn {
            body["bvn"] = json!(bvn);
        }
        if let Some(expiry_date) = expiry_date {
            body["expiryDate"] = json!(expiry_date);
        }
        if let Some(expected_amount) = expected_amount {
            body["expectedAmount"] = json!(expected_amount);
        }

        let path = format!("/v1/accounts/virtual/{}", sub_account_id.into());
        let response = self.client.post(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn filter_virtual_accounts(
        &self,
        limit: Option<String>,
        cursor: Option<String>,
        account_name: Option<String>,
        account_ref: Option<String>,
        bvn: Option<String>,
        bank_account_number: Option<String>,
        date_created_from: Option<String>,
        date_created_to: Option<String>,
        expired: Option<bool>,
        resource_acquired: Option<bool>,
    ) -> Result<FilterVirtualAccountsResponse> {
        let mut params = Vec::new();
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(cursor) = cursor {
            params.push(("cursor", cursor));
        }

        let mut body = json!({});
        if let Some(account_name) = account_name {
            body["accountName"] = json!(account_name);
        }
        if let Some(account_ref) = account_ref {
            body["accountRef"] = json!(account_ref);
        }
        if let Some(bvn) = bvn {
            body["bvn"] = json!(bvn);
        }
        if let Some(bank_account_number) = bank_account_number {
            body["bankAccountNumber"] = json!(bank_account_number);
        }
        if let Some(date_created_from) = date_created_from {
            body["dateCreatedFrom"] = json!(date_created_from);
        }
        if let Some(date_created_to) = date_created_to {
            body["dateCreatedTo"] = json!(date_created_to);
        }
        if let Some(expired) = expired {
            body["expired"] = json!(expired);
        }
        if let Some(resource_acquired) = resource_acquired {
            body["resourceAcquired"] = json!(resource_acquired);
        }

        let response = self
            .client
            .post("/v1/accounts/virtual/list", &body, Some(params))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update_virtual_account(
        &self,
        identifier: impl Into<String>,
        new_account_ref: Option<String>,
        account_name: Option<String>,
        callback_url: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<UpdateVirtualAccountResponse> {
        let mut body = json!({});
        if let Some(new_account_ref) = new_account_ref {
            body["newAccountRef"] = json!(new_account_ref);
        }
        if let Some(account_name) = account_name {
            body["accountName"] = json!(account_name);
        }
        if let Some(callback_url) = callback_url {
            body["callbackUrl"] = json!(callback_url);
        }
        if let Some(expected_amount) = expected_amount {
            body["expectedAmount"] = json!(expected_amount);
        }

        let path = format!("/v1/accounts/virtual/{}", identifier.into());
        let response = self.client.put(&path, &body, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn fetch_virtual_account(
        &self,
        identifier: impl Into<String>,
    ) -> Result<FetchVirtualAccountResponse> {
        let path = format!("/v1/accounts/virtual/{}", identifier.into());
        let response = self.client.get(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn expire_virtual_account(
        &self,
        identifier: impl Into<String>,
    ) -> Result<ExpireVirtualAccountResponse> {
        let path = format!("/v1/accounts/virtual/{}", identifier.into());
        let response = self.client.delete(&path, None).await?;
        Ok(serde_json::from_value(response)?)
    }
}