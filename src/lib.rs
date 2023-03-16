#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#[macro_use]
extern crate derive_builder;

mod api;
mod client;
mod config;
mod path;

pub mod prelude {
    pub use super::{
        api::{
            domains::{
                ApiDomainEndpoints, Domain,
                DomainRequestBuilder, DomainResponse,
            },
            message::{
                attachment::{
                    Attachment,
                    FetchAttachmentRequestBuilder,
                    FetchListOfAttachmentRequestBuilder,
                    FetchListOfAttachmentResponse,
                    LookupField,
                },
                inbox::{
                    FetchInboxRequestBuilder,
                    FetchInboxRequestQueryParamsBuilder,
                    FetchInboxResponse, Msg, Sorting,
                },
                link::{
                    FetchLinkRequestBuilder,
                    FetchLinkResponse,
                },
                ApiMessageEndpoints,
                DeleteAllDomainMessageRequestBuilder,
                DeleteAllInboxMessageRequestBuilder,
                DeleteMessageRequestBuilder,
                DeleteMessageResponse, Email,
                FetchMessageRequestBuilder,
                FetchSmsMessageRequestBuilder,
                InjectMessageRequestBuilder,
                InjectMessageResponse, NewEmail,
                NewEmailBuilder, Part, PartBuilder,
            },
            rules::{
                ApiRuleEndpoints, CreateRuleRequestBuilder,
                ListRulesRequestBuilder, ListRulesResponse,
                RuleBuilder, RuleRequestBuilder,
                WebhookActionBuilder,
            },
            stats::*,
            ResponseStatus,
        },
        client::Mailinator,
    };
}
