use crate::api::rules::Rule;
use crate::api::ResponseStatus;
use crate::client::Mailinator;
use crate::path::AsUrl;
use async_trait::async_trait;
use eyre::Report;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub _id: String,
    pub description: String,
    pub enabled: bool,
    pub name: String,
    pub ownerid: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct DomainResponse {
    pub domains: Vec<Domain>,
}

#[derive(Debug, Builder)]
pub struct DomainRequest {
    domain: String,
}

impl AsUrl for DomainRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain_id}",
            domain_id = self.domain
        )
    }
}

#[async_trait]
pub trait ApiDomainEndpoints {
    async fn get_all_domains(
        &self,
    ) -> Result<DomainResponse, Report>;
    async fn get_domain(
        &self,
        request: DomainRequest,
    ) -> Result<Domain, Report>;
    async fn create_private_domain(
        &self,
        request: DomainRequest,
    ) -> Result<ResponseStatus, Report>;
    async fn delete_private_domain(
        &self,
        request: DomainRequest,
    ) -> Result<ResponseStatus, Report>;
}

#[async_trait]
impl ApiDomainEndpoints for Mailinator {
    async fn get_all_domains(
        &self,
    ) -> Result<DomainResponse, Report> {
        let path = "/domains/";
        self.get(path.to_owned()).await
    }
    async fn get_domain(
        &self,
        request: DomainRequest,
    ) -> Result<Domain, Report> {
        self.get(request.as_url_path()).await
    }
    async fn create_private_domain(
        &self,
        request: DomainRequest,
    ) -> Result<ResponseStatus, Report> {
        self.post(request.as_url_path(), vec![]).await
    }
    async fn delete_private_domain(
        &self,
        request: DomainRequest,
    ) -> Result<ResponseStatus, Report> {
        self.delete(request.as_url_path()).await
    }
}
