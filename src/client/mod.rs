use crate::config::MailinatorConfig;
use eyre::Report;
use futures::TryFutureExt;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, Client, RequestBuilder, Response,
};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct MailinatorClient {
    _client: Client,
    _headers: HeaderMap,
    _api_url: String,
}

impl MailinatorClient {
    pub fn new(_api_url: Option<String>, _api_token: Option<String>) -> Self {
        let mut _headers = HeaderMap::new();
        let MailinatorConfig { api_url, api_token } = MailinatorConfig::new();

        let _api_url = _api_url.unwrap_or(api_url);

        _headers.insert(
            "Authorization",
            HeaderValue::try_from(
                _api_token
                    .or(api_token)
                    .expect("No valid api token were provided"),
            )
            .expect("Cannot build authorization"),
        );

        let _client = Client::builder()
            .build()
            .expect("Failed to build http client");

        Self {
            _client,
            _headers,
            _api_url,
        }
    }

    pub(crate) async fn get<T>(&self, path: String) -> Result<T, Report>
    where
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::get(self, path)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }

    pub(crate) async fn post<Data, T>(&self, path: String, data: Data) -> Result<T, Report>
    where
        Data: Serialize + Sync + Send,
        Data: Into<Body>,
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::post(self, path, data)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }

    pub(crate) async fn post_form<Form, T>(&self, path: String, form: Form) -> Result<T, Report>
    where
        Form: Serialize + Sync + Send,
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::post_form(self, path, form)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }
}

#[allow(dead_code)]
struct HttpRequest;

impl HttpRequest {
    async fn get(inner: &MailinatorClient, path: String) -> Result<RequestBuilder, Report> {
        let MailinatorClient {
            _client,
            _headers,
            _api_url,
        } = inner;
        Ok(_client
            .get(format!("{_api_url}{path}"))
            .headers(_headers.to_owned()))
    }

    async fn post<Data>(
        inner: &MailinatorClient,
        path: String,
        data: Data,
    ) -> Result<RequestBuilder, Report>
    where
        Data: Serialize + Sync + Send,
        Data: Into<Body>,
    {
        let MailinatorClient {
            _client,
            _headers,
            _api_url,
        } = inner;
        Ok(_client
            .post(format!("{_api_url}{path}"))
            .body(data)
            .headers(_headers.to_owned()))
    }

    async fn post_form<Form>(
        inner: &MailinatorClient,
        path: String,
        form: Form,
    ) -> Result<RequestBuilder, Report>
    where
        Form: Serialize + Sync + Send,
    {
        let MailinatorClient {
            _client,
            _headers,
            _api_url,
        } = inner;
        Ok(_client
            .post(format!("{_api_url}{path}"))
            .form(&form)
            .headers(_headers.to_owned()))
    }

    async fn send(req: RequestBuilder) -> Result<Response, Report> {
        match req.send().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(e.into()),
        }
    }

    async fn parse_json<T>(resp: Response) -> Result<T, Report>
    where
        T: DeserializeOwned + Sync + Send,
    {
        match resp.json::<T>().await {
            Ok(j) => Ok(j),
            Err(e) => Err(e.into()),
        }
    }
}
