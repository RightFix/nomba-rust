use serde::Deserialize;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NombaError {
    #[error("API error: {message}")]
    Api {
        message: String,
        status_code: Option<u16>,
        code: Option<String>,
        response_body: Option<serde_json::Value>,
    },

    #[error("Authentication error: {message}")]
    Auth {
        message: String,
        status_code: Option<u16>,
        response_body: Option<serde_json::Value>,
    },

    #[error("Validation error: {message}")]
    Validation {
        message: String,
        missing: Vec<String>,
    },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl NombaError {
    pub fn api(message: impl Into<String>) -> Self {
        Self::Api {
            message: message.into(),
            status_code: None,
            code: None,
            response_body: None,
        }
    }

    pub fn api_with_details(
        message: impl Into<String>,
        status_code: u16,
        code: Option<String>,
        response_body: Option<serde_json::Value>,
    ) -> Self {
        Self::Api {
            message: message.into(),
            status_code: Some(status_code),
            code,
            response_body,
        }
    }

    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth {
            message: message.into(),
            status_code: None,
            response_body: None,
        }
    }

    pub fn auth_with_details(
        message: impl Into<String>,
        status_code: u16,
        response_body: Option<serde_json::Value>,
    ) -> Self {
        Self::Auth {
            message: message.into(),
            status_code: Some(status_code),
            response_body,
        }
    }

    pub fn validation(message: impl Into<String>, missing: Vec<String>) -> Self {
        Self::Validation {
            message: message.into(),
            missing,
        }
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Api { status_code, .. } => {
                matches!(status_code, Some(429 | 500 | 502 | 503 | 504))
            }
            Self::Http(e) => e.is_timeout() || e.is_connect() || e.is_request(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub description: String,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.description)
    }
}

pub type Result<T> = std::result::Result<T, NombaError>;
