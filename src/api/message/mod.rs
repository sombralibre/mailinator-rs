use crate::client::Mailinator;
use crate::path::AsUrl;
use async_trait::async_trait;
use eyre::Report;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod attachment;
pub mod inbox;
pub mod link;

use self::{
    attachment::{
        FetchAttachmentRequest,
        FetchListOfAttachmentRequest,
        FetchListOfAttachmentResponse,
    },
    inbox::{FetchInboxRequest, FetchInboxResponse},
    link::{FetchLinkRequest, FetchLinkResponse},
};

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

impl AsUrl for FetchMessageRequest {
    fn as_url_path(self) -> String {
        let Self {
            domain,
            inbox,
            message_id,
        } = self;
        inbox.map_or_else(|| format!(
            "/api/v2/domains/{domain}/messages/{message_id}"), 
            |ibx|
             format!("/api/v2/domains/{domain}/inboxes/{ibx}/messages/{message_id}"))
    }
}

impl AsUrl for FetchSmsMessageRequest {
    fn as_url_path(self) -> String {
        let Self { domain, sms_number } = self;
        format!(
            "/api/v2/domains/{domain}/inboxes/{sms_number}"
        )
    }
}

impl AsUrl for DeleteAllDomainMessageRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain}/inboxes/",
            domain = self.domain
        )
    }
}

impl AsUrl for DeleteAllInboxMessageRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain}/inboxes/{inbox}",
            domain = self.domain,
            inbox = self.inbox
        )
    }
}

impl AsUrl for DeleteMessageRequest {
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
pub struct Email {
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
pub struct NewEmail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fromfull: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    pub subject: String,
    pub parts: Vec<Part>,
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
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

impl AsUrl for InjectMessageRequest {
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

/// This trait provides the methods for the Mailinator Message Api
///
/// # Errors
/// Any error will be wrapped as a [`eyre::Report`]
#[async_trait]
pub trait ApiMessageEndpoints {
    ///Fetch Inbox (aka Fetch Message Summaries)
    ///
    /// This endpoint retrieves a list of messages summaries. You can retreive a list by inbox, inboxes, or entire domain.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{
    ///    FetchInboxRequestBuilder, FetchInboxRequestQueryParamsBuilder, MailinatorClient, MessageApi,Sorting
    ///};
    ///
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///
    ///    let params = FetchInboxRequestQueryParamsBuilder::default()
    ///        .limit(Some(1))
    ///        .sort(Some(Sorting::Ascending))
    ///        .build()
    ///        .expect("failed to build query params");
    ///
    ///    let request = FetchInboxRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .query_params(Some(params))
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.fetch_inbox(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(msgs) => println!("{msgs:?}"),
    ///    }
    ///}
    ///```
    async fn fetch_inbox(
        &self,
        request: FetchInboxRequest,
    ) -> Result<FetchInboxResponse, Report>;
    /// Fetch Message
    ///
    /// This endpoint retrieves a specific message by id.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{FetchMessageRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let msg_id = String::from("xxx-id-xx-msg");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = FetchMessageRequestBuilder::default()
    ///        .domain(domain)
    ///        .message_id(msg_id)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.fetch_message(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(msg) => println!("{msg:?}"),
    ///    }
    ///}
    ///```
    async fn fetch_message(
        &self,
        request: FetchMessageRequest,
    ) -> Result<Email, Report>;
    /// Fetch an SMS Messages
    ///
    /// SMS messages go into an inbox by the name of their phone number. Retrieving them is the same as any other message,
    /// simply use the phone number as the Inbox you are fetching.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{FetchSmsMessageRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let sms_number: u64 = 12345678;
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = FetchSmsMessageRequestBuilder::default()
    ///        .domain(domain)
    ///        .sms_number(sms_number)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.fetch_sms_message(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(msg) => println!("{msg:?}"),
    ///    }
    ///}
    ///```
    async fn fetch_sms_message(
        &self,
        request: FetchSmsMessageRequest,
    ) -> Result<Email, Report>;
    ///Fetch List of Attachments
    ///
    ///This endpoint retrieves a list of attachments for a message. Note attachments are expected to be in Email format.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{FetchListOfAttachmentRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///    let msg_id = String::from("msg_id");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = FetchListOfAttachmentRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .message_id(msg_id)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.fetch_list_of_attachments(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(at_list) => println!("{at_list:?}"),
    ///    }
    ///}
    ///```
    async fn fetch_list_of_attachments(
        &self,
        request: FetchListOfAttachmentRequest,
    ) -> Result<FetchListOfAttachmentResponse, Report>;
    /// Fetch Links
    ///
    /// This endpoint retrieves all links found within a given email
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{FetchLinkRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///    let msg_id = String::from("msg_id");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = FetchLinkRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .message_id(msg_id)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.fetch_links(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(links) => println!("{links:?}"),
    ///    }
    ///}
    ///
    ///```
    async fn fetch_links(
        &self,
        request: FetchLinkRequest,
    ) -> Result<FetchLinkResponse, Report>;
    /// Fetch Attachment
    ///
    ///This endpoint retrieves a list of attachments for a message. Note attachments are expected to be in Email format.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{
    ///    AttachmentLookupField, FetchAttachmentRequestBuilder, MailinatorClient, MessageApi,
    ///};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///    let msg_id = String::from("msg_id");
    ///    let attachment_id = AttachmentLookupField::Id(1);
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = FetchAttachmentRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .message_id(msg_id)
    ///        .attachment(attachment_id)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.fetch_attachment(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(msg) => println!("{msg:?}"),
    ///    }
    ///}
    ///```
    async fn fetch_attachment(
        &self,
        request: FetchAttachmentRequest,
    ) -> Result<Email, Report>;
    /// Delete ALL Messages (by Domain)
    ///
    /// This endpoint deletes ALL messages from a Private Domain. Caution: This action is irreversible.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{DeleteAllDomainMessageRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = DeleteAllDomainMessageRequestBuilder::default()
    ///        .domain(domain)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.delete_all_domain_messages(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(status) => println!("{status:?}"),
    ///    }
    ///}
    ///```
    async fn delete_all_domain_messages(
        &self,
        request: DeleteAllDomainMessageRequest,
    ) -> Result<DeleteMessageResponse, Report>;
    /// Delete ALL Messages (by Domain)
    ///
    /// This endpoint deletes ALL messages from a Private Domain. Caution: This action is irreversible.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{DeleteAllInboxMessageRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = DeleteAllInboxMessageRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.delete_all_inbox_messages(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(status) => println!("{status:?}"),
    ///    }
    ///}
    ///```
    async fn delete_all_inbox_messages(
        &self,
        request: DeleteAllInboxMessageRequest,
    ) -> Result<DeleteMessageResponse, Report>;
    /// Delete ALL Messages (by Inbox)
    ///
    /// This endpoint deletes ALL messages from a specific private inbox.
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{DeleteMessageRequestBuilder, MailinatorClient, MessageApi};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///    let msg_id = String::from("msg_id");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = DeleteMessageRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .message_id(msg_id)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    match client.delete_message(request).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(status) => println!("{status:?}"),
    ///    }
    ///}
    ///```
    async fn delete_message(
        &self,
        request: DeleteMessageRequest,
    ) -> Result<DeleteMessageResponse, Report>;
    /// Inject a Message (HTTP Post messages)
    ///
    /// This endpoint allows you to deliver a JSON message into your private domain. This is similar to simply emailing a message
    /// to your private domain, except that you use HTTP Post and can programmatically inject the message.
    ///
    /// Note that injected JSON Messages can have any schema they choose. However, if you want the Web interface to display
    /// them, they must follow a general email format with the fields of From, Subject, and Parts (see "Fetch Message" above).
    ///
    /// ```rust
    ///use mailinator_rs::prelude::{
    ///    InjectMessageRequestBuilder, MailinatorClient, MessageApi, NewEmailMessageBuilder,
    ///};
    ///
    ///#[tokio::main]
    ///async fn main() {
    ///    let team_api_token = String::from("XXXXX-XXXXXX");
    ///    let domain = String::from("example.com");
    ///    let inbox = String::from("testinbox");
    ///
    ///    let client = MailinatorClient::new(None, Some(team_api_token));
    ///
    ///    let request = InjectMessageRequestBuilder::default()
    ///        .domain(domain)
    ///        .inbox(inbox)
    ///        .build()
    ///        .expect("failed to build request");
    ///
    ///    let email = NewEmailMessageBuilder::default()
    ///        .subject(String::from("Hello world"))
    ///        .text(Some(String::from("An example email.")))
    ///        .from(String::from("sender@example.com"))
    ///        .build()
    ///        .expect("failed to build email message");
    ///
    ///    match client.inject_message(request, email).await {
    ///        Err(e) => println!("Error: {e}"),
    ///        Ok(status) => println!("{status:?}"),
    ///    }
    ///}
    ///```
    async fn inject_message(
        &self,
        request: InjectMessageRequest,
        email: NewEmail,
    ) -> Result<InjectMessageResponse, Report>;
}

#[async_trait]
impl ApiMessageEndpoints for Mailinator {
    async fn fetch_inbox(
        &self,
        request: FetchInboxRequest,
    ) -> Result<FetchInboxResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_message(
        &self,
        request: FetchMessageRequest,
    ) -> Result<Email, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_sms_message(
        &self,
        request: FetchSmsMessageRequest,
    ) -> Result<Email, Report> {
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
    ) -> Result<Email, Report> {
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
        email: NewEmail,
    ) -> Result<InjectMessageResponse, Report> {
        self.post_json(request.as_url_path(), email).await
    }
}
