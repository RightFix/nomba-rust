use crate::error::Result;
use crate::http_client::BlockingNombaClient;
use crate::http_client::NombaClient;
use crate::models::*;
use serde_json::json;

/// Request body for virtual-account creation. `nin` (11 digits) may be
/// sent on its own or alongside `bvn`; when neither is sent Nomba falls
/// back to the parent business account's BVN.
pub fn build_virtual_account_body(
    account_ref: String,
    account_name: String,
    bvn: Option<String>,
    nin: Option<String>,
    expiry_date: Option<String>,
    expected_amount: Option<String>,
) -> serde_json::Value {
    let mut body = json!({
        "accountRef": account_ref,
        "accountName": account_name,
    });
    if let Some(bvn) = bvn {
        body["bvn"] = json!(bvn);
    }
    if let Some(nin) = nin {
        body["nin"] = json!(nin);
    }
    if let Some(expiry_date) = expiry_date {
        body["expiryDate"] = json!(expiry_date);
    }
    if let Some(expected_amount) = expected_amount {
        body["expectedAmount"] = json!(expected_amount);
    }
    body
}

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
        nin: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let body = build_virtual_account_body(
            account_ref.into(),
            account_name.into(),
            bvn,
            nin,
            expiry_date,
            expected_amount,
        );

        let response = self.client.post("/v1/accounts/virtual", &body, None)?;
        Ok(serde_json::from_value(response)?)
    }

    pub fn create_virtual_account_for_sub_account(
        &self,
        sub_account_id: impl Into<String>,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        bvn: Option<String>,
        nin: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let body = build_virtual_account_body(
            account_ref.into(),
            account_name.into(),
            bvn,
            nin,
            expiry_date,
            expected_amount,
        );

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

        let response = self
            .client
            .post("/v1/accounts/virtual/list", &body, Some(params))?;
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
        nin: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let body = build_virtual_account_body(
            account_ref.into(),
            account_name.into(),
            bvn,
            nin,
            expiry_date,
            expected_amount,
        );

        let response = self
            .client
            .post("/v1/accounts/virtual", &body, None)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create_virtual_account_for_sub_account(
        &self,
        sub_account_id: impl Into<String>,
        account_ref: impl Into<String>,
        account_name: impl Into<String>,
        bvn: Option<String>,
        nin: Option<String>,
        expiry_date: Option<String>,
        expected_amount: Option<String>,
    ) -> Result<CreateVirtualAccountResponse> {
        let body = build_virtual_account_body(
            account_ref.into(),
            account_name.into(),
            bvn,
            nin,
            expiry_date,
            expected_amount,
        );

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

#[cfg(test)]
mod virtual_account_body_tests {
    use super::build_virtual_account_body;

    #[test]
    fn nin_sent_alone() {
        let b = build_virtual_account_body(
            "BS-NOMBA-DVA-1".to_string(),
            "Test User".to_string(),
            None,
            Some("12345678901".to_string()),
            None,
            None,
        );
        assert_eq!(b["nin"], "12345678901");
        assert!(b.get("bvn").is_none());
        assert_eq!(b["accountRef"], "BS-NOMBA-DVA-1");
    }

    #[test]
    fn nin_and_bvn_sent_together() {
        let b = build_virtual_account_body(
            "ref".to_string(),
            "Name Here".to_string(),
            Some("12345678901".to_string()),
            Some("12345678901".to_string()),
            Some("2027-01-30 12:15:00".to_string()),
            Some("200.00".to_string()),
        );
        assert_eq!(b["bvn"], "12345678901");
        assert_eq!(b["nin"], "12345678901");
        assert_eq!(b["expiryDate"], "2027-01-30 12:15:00");
        assert_eq!(b["expectedAmount"], "200.00");
    }

    #[test]
    fn neither_identifier_omits_both() {
        let b = build_virtual_account_body(
            "ref".to_string(),
            "Name Here".to_string(),
            None,
            None,
            None,
            None,
        );
        assert!(b.get("bvn").is_none());
        assert!(b.get("nin").is_none());
    }
}
