use crate::path::AsUrl;
use serde::Deserialize;

#[derive(Debug, Builder, Default)]
pub struct FetchListOfAttachmentRequest {
    domain: String,
    inbox: String,
    message_id: String,
}

impl AsUrl for FetchListOfAttachmentRequest {
    fn as_url_path(self) -> String {
        let Self {
            domain,
            inbox,
            message_id,
        } = self;
        format!("/api/v2/domains/{domain}/inboxes/{inbox}/messages/{message_id}/attachments")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    pub filename: Option<String>,
    #[serde(rename = "content-disposition")]
    pub content_disposition: Option<String>,
    #[serde(rename = "content-transfer-encoding")]
    pub content_transfer_encoding: Option<String>,
    #[serde(rename = "content-type")]
    pub content_type: Option<String>,
    #[serde(rename = "attachment-id")]
    pub attachment_id: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FetchListOfAttachmentResponse {
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone)]
pub enum LookupField {
    Id(u64),
    Name(String),
}

impl Default for LookupField {
    fn default() -> Self {
        Self::Id(0)
    }
}

#[derive(Debug, Builder, Default)]
pub struct FetchAttachmentRequest {
    domain: String,
    inbox: String,
    message_id: String,
    attachment: LookupField,
}

impl AsUrl for FetchAttachmentRequest {
    fn as_url_path(self) -> String {
        let Self {
            domain,
            inbox,
            message_id,
            attachment,
        } = self;
        match attachment {
            LookupField::Id(id) => format!("/api/v2/domains/{domain}/inboxes/{inbox}/messages/{message_id}/attachments/{id}"),
            LookupField::Name(name) => format!("/api/v2/domains/{domain}/inboxes/{inbox}/messages/{message_id}/attachments/{name}")
        }
    }
}
