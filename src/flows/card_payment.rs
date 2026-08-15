use crate::error::Result;
use crate::models::*;
use crate::resources::charge::{AsyncCharge, Charge};
use serde_json::{json, Value, to_value};

const RESPONSE_CODE_SUCCESS: &str = "00";
const RESPONSE_CODE_OTP_REQUIRED: &str = "T0";
const RESPONSE_CODE_3DS_REQUIRED: &str = "S0";

#[derive(Debug, Clone)]
pub struct CardPaymentStep {
    pub raw: Value,
    pub response_code: Option<String>,
    pub status: Option<Value>,
    pub message: Option<String>,
    pub transaction_id: Option<String>,
    pub requires_otp: bool,
    pub requires_3ds: bool,
    pub secure_authentication_data: Option<Value>,
    pub completed: bool,
}

fn interpret(raw: Value, transaction_id_fallback: Option<String>) -> Result<CardPaymentStep> {
    let data = raw.get("data").and_then(|v| v.as_object()).cloned().unwrap_or_default();
    let response_code = data.get("responseCode").and_then(|v| v.as_str()).map(|s| s.to_string());
    let transaction_id = data.get("transactionId").and_then(|v| v.as_str()).map(|s| s.to_string())
        .or(transaction_id_fallback);

    Ok(CardPaymentStep {
        raw: raw.clone(),
        response_code: response_code.clone(),
        status: data.get("status").cloned(),
        message: data.get("message").and_then(|v| v.as_str()).map(|s| s.to_string()),
        transaction_id,
        requires_otp: response_code.as_deref() == Some(RESPONSE_CODE_OTP_REQUIRED),
        requires_3ds: response_code.as_deref() == Some(RESPONSE_CODE_3DS_REQUIRED),
        secure_authentication_data: data.get("secureAuthenticationData").cloned(),
        completed: response_code.as_deref() == Some(RESPONSE_CODE_SUCCESS),
    })
}

pub struct CardPaymentFlow {
    charge: Charge,
    order_reference: String,
    transaction_id: Option<String>,
}

impl CardPaymentFlow {
    pub fn new(charge: Charge, order_reference: String) -> Self {
        Self {
            charge,
            order_reference,
            transaction_id: None,
        }
    }

    pub fn submit_card(
        &mut self,
        card_details: impl Into<String>,
        key: impl Into<String>,
        save_card: Option<bool>,
        device_information: Option<Value>,
    ) -> Result<CardPaymentStep> {
        let raw = self.charge.submit_customer_card_details(
            self.order_reference.clone(),
            card_details,
            key,
            save_card,
            device_information,
        )?;
        let raw_value = to_value(&raw)?;
        let step = interpret(raw_value, self.transaction_id.clone())?;
        self.transaction_id = step.transaction_id.clone();
        Ok(step)
    }

    pub fn submit_otp(&mut self, otp: impl Into<String>) -> Result<CardPaymentStep> {
        let transaction_id = self.transaction_id.as_ref()
            .ok_or_else(|| crate::error::NombaError::validation(
                "No transaction_id on this flow yet — call submit_card() first.".to_string(),
                vec![],
            ))?;

        let raw = self.charge.submit_customer_payment_otp(
            self.order_reference.clone(),
            otp,
            transaction_id.clone(),
        )?;
        let raw_value = to_value(&raw)?;
        interpret(raw_value, self.transaction_id.clone())
    }

    pub fn resend_otp(&self) -> Result<ResendOtpResponse> {
        self.charge.resend_customer_payment_otp(self.order_reference.clone())
    }

    pub fn confirm(&self) -> Result<FetchCheckoutTransactionDetailsResponse> {
        self.charge.fetch_checkout_transaction_details(self.order_reference.clone())
    }

    pub fn cancel(&self, force: bool) -> Result<CancelCheckoutTransactionResponse> {
        let transaction_id = self.transaction_id.as_ref()
            .ok_or_else(|| crate::error::NombaError::validation(
                "No transaction_id on this flow yet — call submit_card() first.".to_string(),
                vec![],
            ))?;
        self.charge.cancel_checkout_transaction(self.order_reference.clone(), transaction_id.clone(), Some(force))
    }
}

pub struct AsyncCardPaymentFlow {
    charge: AsyncCharge,
    order_reference: String,
    transaction_id: Option<String>,
}

impl AsyncCardPaymentFlow {
    pub fn new(charge: AsyncCharge, order_reference: String) -> Self {
        Self {
            charge,
            order_reference,
            transaction_id: None,
        }
    }

    pub async fn submit_card(
        &mut self,
        card_details: impl Into<String>,
        key: impl Into<String>,
        save_card: Option<bool>,
        device_information: Option<Value>,
    ) -> Result<CardPaymentStep> {
        let raw = self.charge.submit_customer_card_details(
            self.order_reference.clone(),
            card_details,
            key,
            save_card,
            device_information,
        ).await?;
        let raw_value = to_value(&raw)?;
        let step = interpret(raw_value, self.transaction_id.clone())?;
        self.transaction_id = step.transaction_id.clone();
        Ok(step)
    }

    pub async fn submit_otp(&mut self, otp: impl Into<String>) -> Result<CardPaymentStep> {
        let transaction_id = self.transaction_id.as_ref()
            .ok_or_else(|| crate::error::NombaError::validation(
                "No transaction_id on this flow yet — call submit_card() first.".to_string(),
                vec![],
            ))?;

        let raw = self.charge.submit_customer_payment_otp(
            self.order_reference.clone(),
            otp,
            transaction_id.clone(),
        ).await?;
        let raw_value = to_value(&raw)?;
        interpret(raw_value, self.transaction_id.clone())
    }

    pub async fn resend_otp(&self) -> Result<ResendOtpResponse> {
        self.charge.resend_customer_payment_otp(self.order_reference.clone()).await
    }

    pub async fn confirm(&self) -> Result<FetchCheckoutTransactionDetailsResponse> {
        self.charge.fetch_checkout_transaction_details(self.order_reference.clone()).await
    }

    pub async fn cancel(&self, force: bool) -> Result<CancelCheckoutTransactionResponse> {
        let transaction_id = self.transaction_id.as_ref()
            .ok_or_else(|| crate::error::NombaError::validation(
                "No transaction_id on this flow yet — call submit_card() first.".to_string(),
                vec![],
            ))?;
        self.charge.cancel_checkout_transaction(self.order_reference.clone(), transaction_id.clone(), Some(force)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_interpret_success() {
        let raw = json!({
            "data": {
                "responseCode": "00",
                "transactionId": "txn-123",
                "status": true,
                "message": "Success"
            }
        });
        let step = interpret(raw, None).unwrap();
        assert!(step.completed);
        assert!(!step.requires_otp);
        assert!(!step.requires_3ds);
        assert_eq!(step.transaction_id, Some("txn-123".to_string()));
    }

    #[test]
    fn test_interpret_otp_required() {
        let raw = json!({
            "data": {
                "responseCode": "T0",
                "transactionId": "txn-123",
                "status": false,
                "message": "OTP required"
            }
        });
        let step = interpret(raw, None).unwrap();
        assert!(!step.completed);
        assert!(step.requires_otp);
        assert!(!step.requires_3ds);
        assert_eq!(step.transaction_id, Some("txn-123".to_string()));
    }

    #[test]
    fn test_interpret_3ds_required() {
        let raw = json!({
            "data": {
                "responseCode": "S0",
                "transactionId": "txn-123",
                "status": false,
                "message": "3DS required",
                "secureAuthenticationData": {"url": "https://example.com"}
            }
        });
        let step = interpret(raw, None).unwrap();
        assert!(!step.completed);
        assert!(!step.requires_otp);
        assert!(step.requires_3ds);
        assert!(step.secure_authentication_data.is_some());
    }
}