use crate::error::{NombaError, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;
use std::time::Duration;

type HmacSha256 = Hmac<Sha256>;

const SIGNATURE_HEADER: &str = "nomba-signature";
const TIMESTAMP_HEADER: &str = "nomba-timestamp";

fn get_path(payload: &Value, path: &[&str], default: &str) -> String {
    let mut current = payload;
    for key in path {
        match current.get(*key) {
            Some(v) => current = v,
            None => return default.to_string(),
        }
    }
    current.as_str().unwrap_or(default).to_string()
}

fn parse_rfc3339(timestamp: &str) -> Result<DateTime<Utc>> {
    let ts = timestamp.trim();
    let ts = if ts.ends_with('Z') {
        format!("{} +00:00", &ts[..ts.len() - 1])
    } else {
        ts.to_string()
    };
    DateTime::parse_from_rfc3339(&ts)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| NombaError::validation(format!("Invalid timestamp format: {}", e), vec![]))
}

pub fn check_timestamp_freshness(timestamp: &str, max_age_seconds: f64) -> Result<()> {
    let sent_at = parse_rfc3339(timestamp)?;
    let now = Utc::now();
    let age = (now - sent_at).num_seconds() as f64;

    if age > max_age_seconds {
        return Err(NombaError::validation(
            format!(
                "Webhook timestamp is {:.0}s old, exceeding max_age_seconds={} (possible replay attack)",
                age, max_age_seconds
            ),
            vec![],
        ));
    }

    if age < -max_age_seconds {
        return Err(NombaError::validation(
            format!(
                "Webhook timestamp is {:.0}s in the future, exceeding max_age_seconds={} (clock skew?)",
                -age, max_age_seconds
            ),
            vec![],
        ));
    }

    Ok(())
}

pub fn compute_signature(signature_key: &str, payload: &Value, timestamp: &str) -> String {
    let hashing_payload = [
        get_path(payload, &["event_type"], ""),
        get_path(payload, &["requestId"], ""),
        get_path(payload, &["data", "merchant", "userId"], ""),
        get_path(payload, &["data", "merchant", "walletId"], ""),
        get_path(payload, &["data", "transaction", "transactionId"], ""),
        get_path(payload, &["data", "transaction", "type"], ""),
        get_path(payload, &["data", "transaction", "time"], ""),
        get_path(payload, &["data", "transaction", "responseCode"], ""),
    ]
    .join(":");

    let message = format!("{}:{}", hashing_payload, timestamp);

    let mut mac = HmacSha256::new_from_slice(signature_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();

    general_purpose::STANDARD.encode(result)
}

pub fn verify_webhook_signature(
    signature_key: &str,
    payload: &Value,
    signature: &str,
    timestamp: &str,
) -> bool {
    let expected = compute_signature(signature_key, payload, timestamp);
    // Constant-time comparison
    use subtle::ConstantTimeEq;
    expected.as_bytes().ct_eq(signature.as_bytes()).into()
}

pub fn verify_webhook_request(
    signature_key: &str,
    body: &[u8],
    headers: &std::collections::HashMap<String, String>,
    max_age_seconds: Option<f64>,
) -> Result<Value> {
    let signature = headers.get(SIGNATURE_HEADER).ok_or_else(|| {
        NombaError::validation(format!("Missing '{}' header", SIGNATURE_HEADER), vec![])
    })?;

    let timestamp = headers.get(TIMESTAMP_HEADER).ok_or_else(|| {
        NombaError::validation(format!("Missing '{}' header", TIMESTAMP_HEADER), vec![])
    })?;

    if let Some(max_age) = max_age_seconds {
        check_timestamp_freshness(timestamp, max_age)?;
    }

    let payload: Value = serde_json::from_slice(body)
        .map_err(|e| NombaError::validation(format!("Invalid JSON body: {}", e), vec![]))?;

    if !verify_webhook_signature(signature_key, &payload, signature, timestamp) {
        return Err(NombaError::validation(
            "Webhook signature verification failed",
            vec![],
        ));
    }

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_compute_signature() {
        let signature_key = "test-secret-key";
        let payload = json!({
            "event_type": "transaction.success",
            "requestId": "req-123",
            "data": {
                "merchant": {
                    "userId": "user-1",
                    "walletId": "wallet-1"
                },
                "transaction": {
                    "transactionId": "txn-1",
                    "type": "payment",
                    "time": "2024-01-01T00:00:00Z",
                    "responseCode": "00"
                }
            }
        });
        let timestamp = "2024-01-01T00:00:00Z";

        let sig = compute_signature(signature_key, &payload, timestamp);
        assert!(!sig.is_empty());
        assert!(verify_webhook_signature(
            signature_key,
            &payload,
            &sig,
            timestamp
        ));
    }

    #[test]
    fn test_verify_webhook_signature_wrong_key() {
        let payload = json!({});
        let timestamp = "2024-01-01T00:00:00Z";
        let sig = compute_signature("key1", &payload, timestamp);
        assert!(!verify_webhook_signature("key2", &payload, &sig, timestamp));
    }

    #[test]
    fn test_check_timestamp_freshness() {
        let now = Utc::now().to_rfc3339();
        assert!(check_timestamp_freshness(&now, 300.0).is_ok());

        let old = (Utc::now() - chrono::Duration::seconds(600)).to_rfc3339();
        assert!(check_timestamp_freshness(&old, 300.0).is_err());

        let future = (Utc::now() + chrono::Duration::seconds(600)).to_rfc3339();
        assert!(check_timestamp_freshness(&future, 300.0).is_err());
    }
}
