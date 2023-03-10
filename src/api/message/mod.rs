use crate::path::AsUrlPath;
use serde::Deserialize;
use std::collections::HashMap;

pub mod attachment;
pub mod inbox;
pub mod link;

#[derive(Debug, Builder, Default)]
pub struct FetchMessageRequest {
    domain: String,
    inbox: Option<String>,
    message_id: String,
}

#[derive(Debug, Builder, Default)]
pub struct FetchSmsMessageRequest {
    domain: String,
    sms_number: u64,
}

impl AsUrlPath for FetchMessageRequest {
    fn as_url_path(self) -> String {
        let Self {
            domain,
            inbox,
            message_id,
        } = self;
        if let Some(ibx) = inbox {
            format!("/api/v2/domains/{domain}/inboxes/{ibx}/messages/{message_id}")
        } else {
            format!("/api/v2/domains/{domain}/messages/{message_id}")
        }
    }
}

impl AsUrlPath for FetchSmsMessageRequest {
    fn as_url_path(self) -> String {
        let Self { domain, sms_number } = self;
        format!(
            "/api/v2/domains/{domain}/inboxes/{sms_number}"
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailMessageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromfull: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_ago: Option<u64>,
}
