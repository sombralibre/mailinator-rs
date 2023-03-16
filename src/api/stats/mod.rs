use crate::client::Mailinator;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::Report;
use serde::Deserialize;

#[async_trait]
pub trait ApiStatEndpoints {
    async fn get_usage_statistica(
        &self,
    ) -> Result<UsageStatistica, Report>;
}

#[async_trait]
impl ApiStatEndpoints for Mailinator {
    async fn get_usage_statistica(
        &self,
    ) -> Result<UsageStatistica, Report> {
        let url_path = "/api/v2/team/stats";
        self.get(url_path.to_owned()).await
    }
}

#[derive(Debug, Deserialize)]
pub struct Sent {
    pub sms: u64,
    pub email: u64,
}

#[derive(Debug, Deserialize)]
pub struct Retrieved {
    pub web_private: u64,
    pub web_public: u64,
    pub api_email: u64,
    pub api_error: u64,
}

#[derive(Debug, Deserialize)]
pub struct Stats {
    pub date: DateTime<Utc>,
    pub retrieved: Retrieved,
    pub sent: Sent,
}

#[derive(Debug, Deserialize)]
pub struct UsageStatistica {
    pub stats: Vec<Stats>,
}
