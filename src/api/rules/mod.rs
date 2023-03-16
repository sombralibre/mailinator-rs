use crate::client::Mailinator;
use crate::path::AsUrl;
use async_trait::async_trait;
use eyre::Report;
use serde::{Deserialize, Serialize};

use super::ResponseStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConditionOperation {
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "PREFIX")]
    Prefix,
}

#[derive(Debug, Deserialize, Builder, Clone, Serialize)]
pub struct ConditionData {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Builder, Clone, Serialize)]
pub struct Condition {
    pub operation: ConditionOperation,
    pub condition_data: ConditionData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    #[serde(rename = "WEBHOOK")]
    Webhook,
    #[serde(rename = "DROP")]
    Drop,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct WebhookAction {
    pub url: String,
}

#[derive(Debug, Deserialize, Builder, Clone, Serialize)]
pub struct ConditionAction {
    pub action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_data: Option<WebhookAction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConditionMatch {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "ALWAYS")]
    Always,
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct Rule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub priority: u32,
    pub conditions: Vec<Condition>,
    pub actions: Vec<ConditionAction>,
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct CreateRuleRequest {
    domain: String,
}

impl AsUrl for CreateRuleRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain_id}/rules/",
            domain_id = self.domain
        )
    }
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct EnableRuleRequest {
    domain_id: String,
    rule_id: String,
}

impl AsUrl for EnableRuleRequest {
    fn as_url_path(self) -> String {
        let Self { domain_id, rule_id } = self;
        format!("/api/v2/domains/{domain_id}/rules/{rule_id}?action=enable")
    }
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct DisableRuleRequest {
    domain_id: String,
    rule_id: String,
}

impl AsUrl for DisableRuleRequest {
    fn as_url_path(self) -> String {
        let Self { domain_id, rule_id } = self;
        format!("/api/v2/domains/{domain_id}/rules/{rule_id}?action=disable")
    }
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct ListRulesRequest {
    domain_id: String,
}

impl AsUrl for ListRulesRequest {
    fn as_url_path(self) -> String {
        let Self { domain_id } = self;
        format!("/api/v2/domains/{domain_id}/rules/")
    }
}

#[derive(Debug, Deserialize, Builder, Serialize)]
pub struct RuleRequest {
    domain_id: String,
    rule_id: String,
}

impl AsUrl for RuleRequest {
    fn as_url_path(self) -> String {
        let Self { domain_id, rule_id } = self;
        format!(
            "/api/v2/domains/{domain_id}/rules/{rule_id}"
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct ListRulesResponse {
    pub rules: Vec<Rule>,
}

#[async_trait]
pub trait ApiRuleEndpoints {
    async fn create_rule(
        &self,
        request: CreateRuleRequest,
        data: Rule,
    ) -> Result<Rule, Report>;
    async fn enable_rule(
        &self,
        request: EnableRuleRequest,
    ) -> Result<ResponseStatus, Report>;
    async fn disable_rule(
        &self,
        request: DisableRuleRequest,
    ) -> Result<ResponseStatus, Report>;
    async fn list_rules(
        &self,
        request: ListRulesRequest,
    ) -> Result<ListRulesResponse, Report>;
    async fn get_rule(
        &self,
        request: RuleRequest,
    ) -> Result<Rule, Report>;
    async fn delete_rule(
        &self,
        request: RuleRequest,
    ) -> Result<Rule, Report>;
}

#[async_trait]
impl ApiRuleEndpoints for Mailinator {
    async fn create_rule(
        &self,
        request: CreateRuleRequest,
        data: Rule,
    ) -> Result<Rule, Report> {
        self.post_json(request.as_url_path(), data).await
    }
    async fn enable_rule(
        &self,
        request: EnableRuleRequest,
    ) -> Result<ResponseStatus, Report> {
        self.put(request.as_url_path()).await
    }
    async fn disable_rule(
        &self,
        request: DisableRuleRequest,
    ) -> Result<ResponseStatus, Report> {
        self.put(request.as_url_path()).await
    }
    async fn list_rules(
        &self,
        request: ListRulesRequest,
    ) -> Result<ListRulesResponse, Report> {
        self.get(request.as_url_path()).await
    }
    async fn get_rule(
        &self,
        request: RuleRequest,
    ) -> Result<Rule, Report> {
        self.get(request.as_url_path()).await
    }
    async fn delete_rule(
        &self,
        request: RuleRequest,
    ) -> Result<Rule, Report> {
        self.delete(request.as_url_path()).await
    }
}
