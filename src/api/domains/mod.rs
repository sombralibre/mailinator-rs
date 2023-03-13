use crate::api::rules::Rule;
use crate::client::MailinatorClient;
use crate::path::AsUrlPath;
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

#[derive(Debug, Deserialize)]
pub struct DomainResponseStatus {
    pub status: String,
}

#[derive(Debug, Builder)]
pub struct DomainRequest {
    domain: String,
}

impl AsUrlPath for DomainRequest {
    fn as_url_path(self) -> String {
        format!(
            "/api/v2/domains/{domain_id}",
            domain_id = self.domain
        )
    }
}

#[async_trait]
pub trait DomainApi {
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
    ) -> Result<DomainResponseStatus, Report>;
    async fn delete_private_domain(
        &self,
        request: DomainRequest,
    ) -> Result<DomainResponseStatus, Report>;
}

#[async_trait]
impl DomainApi for MailinatorClient {
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
    ) -> Result<DomainResponseStatus, Report> {
        self.post(request.as_url_path(), vec![]).await
    }
    async fn delete_private_domain(
        &self,
        request: DomainRequest,
    ) -> Result<DomainResponseStatus, Report> {
        self.delete(request.as_url_path()).await
    }
}
