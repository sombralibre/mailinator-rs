use crate::client::MailinatorClient;
use crate::path::AsUrlPath;
use async_trait::async_trait;
use eyre::Report;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod attachment;
pub mod inbox;
pub mod link;

use self::{attachment::*, inbox::*, link::*};

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

#[derive(Debug, Builder, Default)]
pub struct DeleteAllDomainMessageRequest {
    domain: String,
}

#[derive(Debug, Builder, Default)]
pub struct DeleteAllInboxMessageRequest {
    domain: String,
    inbox: String,
}

#[derive(Debug, Builder, Default)]
pub struct DeleteMessageRequest {
    domain: String,
    inbox: String,
    message_id: String,
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

impl AsUrlPath for DeleteAllDomainMessageRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain}/inboxes/",
            domain = self.domain
        )
    }
}

impl AsUrlPath for DeleteAllInboxMessageRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain}/inboxes/{inbox}",
            domain = self.domain,
            inbox = self.inbox
        )
    }
}

impl AsUrlPath for DeleteMessageRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain}/inboxes/{inbox}/messages/{message_id}",
            domain = self.domain,
            inbox = self.inbox,
            message_id = self.message_id
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailMessage {
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

#[derive(Debug, Clone, Serialize, Builder)]
pub struct NewEmailMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromfull: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    pub subject: String,
    pub parts: Vec<Part>,
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_ago: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMessageResponse {
    pub status: String,
    pub messages_deleted: u64,
}

#[derive(Debug, Clone, Builder)]
pub struct InjectMessageRequest {
    pub domain: String,
    pub inbox: String,
}

impl AsUrlPath for InjectMessageRequest {
    fn as_url_path(self) -> String {
        let Self { domain, inbox } = self;
        format!("/api/v2/domains/{domain}/inboxes/{inbox}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InjectMessageResponse {
    pub status: String,
    pub id: String,
}

#[async_trait]
pub trait MessageApi {
    async fn fetch_inbox(
        &self,
        request: FetchInboxRequest,
    ) -> Result<FetchInboxResponse, Report>;
    async fn fetch_message(
        &self,
        request: FetchMessageRequest,
    ) -> Result<EmailMessage, Report>;
    async fn fetch_sms_message(
        &self,
        request: FetchSmsMessageRequest,
    ) -> Result<EmailMessage, Report>;
    async fn fetch_list_of_attachments(
        &self,
        request: FetchListOfAttachmentRequest,
    ) -> Result<FetchListOfAttachmentResponse, Report>;
    async fn fetch_links(
        &self,
        request: FetchLinkRequest,
    ) -> Result<FetchLinkResponse, Report>;
    async fn fetch_attachment(
        &self,
        request: FetchAttachmentRequest,
    ) -> Result<EmailMessage, Report>;
    async fn delete_all_domain_messages(
        &self,
        request: DeleteAllDomainMessageRequest,
    ) -> Result<DeleteMessageResponse, Report>;
    async fn delete_all_inbox_messages(
        &self,
        request: DeleteAllInboxMessageRequest,
    ) -> Result<DeleteMessageResponse, Report>;
    async fn delete_message(
        &self,
        request: DeleteMessageRequest,
    ) -> Result<DeleteMessageResponse, Report>;
    async fn inject_message(
        &self,
        request: InjectMessageRequest,
        email: NewEmailMessage,
    ) -> Result<InjectMessageResponse, Report>;
}

#[async_trait]
impl MessageApi for MailinatorClient {
    async fn fetch_inbox(
        &self,
        request: FetchInboxRequest,
    ) -> Result<FetchInboxResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_message(
        &self,
        request: FetchMessageRequest,
    ) -> Result<EmailMessage, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_sms_message(
        &self,
        request: FetchSmsMessageRequest,
    ) -> Result<EmailMessage, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_list_of_attachments(
        &self,
        request: FetchListOfAttachmentRequest,
    ) -> Result<FetchListOfAttachmentResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_links(
        &self,
        request: FetchLinkRequest,
    ) -> Result<FetchLinkResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_attachment(
        &self,
        request: FetchAttachmentRequest,
    ) -> Result<EmailMessage, Report> {
        // TODO: implement a trait helper for file extraction
        self.get(request.as_url_path()).await
    }
    async fn delete_all_domain_messages(
        &self,
        request: DeleteAllDomainMessageRequest,
    ) -> Result<DeleteMessageResponse, Report> {
        self.delete(request.as_url_path()).await
    }
    async fn delete_all_inbox_messages(
        &self,
        request: DeleteAllInboxMessageRequest,
    ) -> Result<DeleteMessageResponse, Report> {
        self.delete(request.as_url_path()).await
    }
    async fn delete_message(
        &self,
        request: DeleteMessageRequest,
    ) -> Result<DeleteMessageResponse, Report> {
        self.delete(request.as_url_path()).await
    }
    async fn inject_message(
        &self,
        request: InjectMessageRequest,
        email: NewEmailMessage,
    ) -> Result<InjectMessageResponse, Report> {
        self.post_json(request.as_url_path(), email).await
    }
}
