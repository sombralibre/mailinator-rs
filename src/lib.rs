#[macro_use]
extern crate derive_builder;

use async_trait::async_trait;
use eyre::Report;

mod api;
mod client;
mod config;
mod path;

pub use self::{
    api::message::{
        attachment::{
            Attachment, AttachmentLookupField,
            FetchAttachmentRequestBuilder,
            FetchListOfAttachmentRequestBuilder,
            FetchListOfAttachmentResponse,
        },
        inbox::{
            FetchInboxRequestBuilder,
            FetchInboxRequestQueryParamsBuilder,
            FetchInboxResponse, Msg,
        },
        link::{
            FetchLinkRequestBuilder, FetchLinkResponse,
        },
        EmailMessageResponse, FetchMessageRequestBuilder,
        FetchSmsMessageRequestBuilder, Part,
    },
    client::MailinatorClient,
};
use self::{
    api::message::{
        attachment::{
            FetchAttachmentRequest,
            FetchListOfAttachmentRequest,
        },
        inbox::FetchInboxRequest,
        link::FetchLinkRequest,
        FetchMessageRequest, FetchSmsMessageRequest,
    },
    path::AsUrlPath,
};

#[async_trait]
pub trait Mailinator {
    async fn fetch_inbox(
        &self,
        request: FetchInboxRequest,
    ) -> Result<FetchInboxResponse, Report>;
    async fn fetch_message(
        &self,
        request: FetchMessageRequest,
    ) -> Result<EmailMessageResponse, Report>;
    async fn fetch_sms_message(
        &self,
        request: FetchSmsMessageRequest,
    ) -> Result<EmailMessageResponse, Report>;
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
    ) -> Result<EmailMessageResponse, Report>;
}

#[async_trait]
impl Mailinator for MailinatorClient {
    async fn fetch_inbox(
        &self,
        request: FetchInboxRequest,
    ) -> Result<FetchInboxResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_message(
        &self,
        request: FetchMessageRequest,
    ) -> Result<EmailMessageResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn fetch_sms_message(
        &self,
        request: FetchSmsMessageRequest,
    ) -> Result<EmailMessageResponse, Report> {
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
    ) -> Result<EmailMessageResponse, Report> {
        // TODO: implement a file extractor
        self.get(request.as_url_path()).await
    }
}
