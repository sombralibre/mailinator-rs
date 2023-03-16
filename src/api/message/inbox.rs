use crate::path::AsUrl;
use serde::{Deserialize, Serialize};
use serde_qs as qs;

#[derive(Debug, Serialize, Clone)]
pub enum Sorting {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

#[derive(Debug, Builder, Default, Serialize, Clone)]
pub struct FetchInboxRequestQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Sorting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decode_subject: Option<bool>,
}

#[derive(Debug, Builder, Default)]
pub struct FetchInboxRequest {
    domain: String,
    inbox: String,
    query_params: Option<FetchInboxRequestQueryParams>,
}

impl AsUrl for FetchInboxRequest {
    fn as_url_path(self) -> String {
        let Self {
            domain,
            inbox,
            query_params,
        } = self;

        query_params.map_or_else(|| format!(
            "/api/v2/domains/{domain}/inboxes/{inbox}"
            ), |q|{
            let qsp = qs::to_string(&q).expect(
                "Failed to serialize query string params",);
            format!("/api/v2/domains/{domain}/inboxes/{inbox}?{qsp}")
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Msg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_ago: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FetchInboxResponse {
    pub domain: String,
    pub to: String,
    pub msgs: Vec<Msg>,
}
