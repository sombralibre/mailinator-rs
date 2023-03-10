use crate::path::AsUrlPath;
use serde::Deserialize;

#[derive(Debug, Builder, Default)]
pub struct FetchLinkRequest {
    domain: String,
    inbox: String,
    message_id: String,
}

impl AsUrlPath for FetchLinkRequest {
    fn as_url_path(self) -> String {
        let Self {
            domain,
            inbox,
            message_id,
        } = self;
        format!("/api/v2/domains/{domain}/inboxes/{inbox}/messages/{message_id}/links")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct FetchLinkResponse {
    pub links: Vec<String>,
}
