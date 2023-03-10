#[macro_use]
extern crate derive_builder;

use async_trait::async_trait;
use eyre::Report;

mod api;
mod client;
mod config;
mod path;

use self::{
    api::message::{inbox::FetchInboxRequest, message::FetchMessageRequest},
    path::AsPath,
};
pub use self::{
    api::message::{
        inbox::{
            FetchInboxRequestBuilder, FetchInboxRequestQueryParamsBuilder, FetchInboxResponse, Msg,
        },
        message::{FetchMessageRequestBuilder, FetchMessageResponse, Part},
    },
    client::MailinatorClient,
};

#[async_trait]
pub trait Mailinator {
    async fn fetch_inbox(&self, req: FetchInboxRequest) -> Result<FetchInboxResponse, Report>;
    async fn fetch_message(&self, req: FetchMessageRequest)
        -> Result<FetchMessageResponse, Report>;
}

#[async_trait]
impl Mailinator for MailinatorClient {
    async fn fetch_inbox(&self, req: FetchInboxRequest) -> Result<FetchInboxResponse, Report> {
        self.get(req.as_path()).await
    }
    async fn fetch_message(
        &self,
        req: FetchMessageRequest,
    ) -> Result<FetchMessageResponse, Report> {
        self.get(req.as_path()).await
    }
}
