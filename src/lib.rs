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
                Domain, DomainRequestBuilder,
                DomainResponse, DomainsApi,
            },
            message::{
                attachment::{
                    Attachment, AttachmentLookupField,
                    FetchAttachmentRequestBuilder,
                    FetchListOfAttachmentRequestBuilder,
                    FetchListOfAttachmentResponse,
                },
                inbox::{
                    FetchInboxRequestBuilder,
                    FetchInboxRequestQueryParamsBuilder,
                    FetchInboxResponse, Msg, Sorting
                },
                link::{
                    FetchLinkRequestBuilder,
                    FetchLinkResponse,
                },
                DeleteAllDomainMessageRequestBuilder,
                DeleteAllInboxMessageRequestBuilder,
                DeleteMessageRequestBuilder,
                DeleteMessageResponse, EmailMessage,
                FetchMessageRequestBuilder,
                FetchSmsMessageRequestBuilder,
                InjectMessageRequestBuilder,
                InjectMessageResponse, MessageApi,
                NewEmailMessage, NewEmailMessageBuilder,
                Part, PartBuilder,
            },
            rules::{
                CreateRuleRequestBuilder,
                ListRulesRequestBuilder, ListRulesResponse,
                RuleBuilder, RuleRequestBuilder, RulesApi,
                WebhookActionBuilder,
            },
            stats::*,
            ResponseStatus,
        },
        client::MailinatorClient,
    };
}
