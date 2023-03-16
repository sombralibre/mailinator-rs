use crate::config::EnvCfg;
use eyre::Report;
use futures::TryFutureExt;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, Client, RequestBuilder, Response,
};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct Mailinator {
    client: Client,
    headers: HeaderMap,
    api_url: String,
}

impl Mailinator {
    #[must_use]
    pub fn new(
        apiurl: Option<String>,
        apitoken: Option<String>,
    ) -> Self {
        let mut headers = HeaderMap::new();
        let EnvCfg { api_url, api_token } = EnvCfg::new();

        let new_api_url = apiurl.unwrap_or(api_url);

        headers.insert(
            "Authorization",
            HeaderValue::try_from(
                apitoken.or(api_token).expect(
                    "No valid api token were provided",
                ),
            )
            .expect("Cannot build authorization"),
        );

        let client = Client::builder()
            .build()
            .expect("Failed to build http client");

        Self {
            client,
            headers,
            api_url: new_api_url,
        }
    }

    pub(crate) async fn get<T>(
        &self,
        path: String,
    ) -> Result<T, Report>
    where
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::get(self, path)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }

    pub(crate) async fn put<T>(
        &self,
        path: String,
    ) -> Result<T, Report>
    where
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::put(self, path)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }

    pub(crate) async fn post<Data, T>(
        &self,
        path: String,
        data: Data,
    ) -> Result<T, Report>
    where
        Data: Serialize + Sync + Send + Into<Body>,
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::post(self, path, data)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }

    pub(crate) async fn post_json<Jdata, T>(
        &self,
        path: String,
        jdata: Jdata,
    ) -> Result<T, Report>
    where
        Jdata: Serialize + Sync + Send,
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::post_json(self, path, jdata)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }

    pub(crate) async fn delete<T>(
        &self,
        path: String,
    ) -> Result<T, Report>
    where
        T: DeserializeOwned + Sync + Send,
    {
        HttpRequest::delete(self, path)
            .and_then(HttpRequest::send)
            .and_then(HttpRequest::parse_json)
            .await
    }
}

#[allow(dead_code)]
struct HttpRequest;

impl HttpRequest {
    async fn get(
        inner: &Mailinator,
        path: String,
    ) -> Result<RequestBuilder, Report> {
        let Mailinator {
            client,
            headers,
            api_url,
        } = inner;
        Ok(client
            .get(format!("{api_url}{path}"))
            .headers(headers.clone()))
    }

    async fn put(
        inner: &Mailinator,
        path: String,
    ) -> Result<RequestBuilder, Report> {
        let Mailinator {
            client,
            headers,
            api_url,
        } = inner;
        Ok(client
            .put(format!("{api_url}{path}"))
            .headers(headers.clone()))
    }

    async fn post<Data>(
        inner: &Mailinator,
        path: String,
        data: Data,
    ) -> Result<RequestBuilder, Report>
    where
        Data: Serialize + Sync + Send + Into<Body>,
    {
        let Mailinator {
            client,
            headers,
            api_url,
        } = inner;
        Ok(client
            .post(format!("{api_url}{path}"))
            .body(data)
            .headers(headers.clone()))
    }

    async fn post_json<Jdata>(
        inner: &Mailinator,
        path: String,
        jdata: Jdata,
    ) -> Result<RequestBuilder, Report>
    where
        Jdata: Serialize + Sync + Send,
    {
        let Mailinator {
            client,
            headers,
            api_url,
        } = inner;
        Ok(client
            .post(format!("{api_url}{path}"))
            .json(&jdata)
            .headers(headers.clone()))
    }

    async fn delete(
        inner: &Mailinator,
        path: String,
    ) -> Result<RequestBuilder, Report> {
        let Mailinator {
            client,
            headers,
            api_url,
        } = inner;
        Ok(client
            .delete(format!("{api_url}{path}"))
            .headers(headers.clone()))
    }

    async fn send(
        req: RequestBuilder,
    ) -> Result<Response, Report> {
        match req.send().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(e.into()),
        }
    }

    async fn parse_json<T>(
        resp: Response,
    ) -> Result<T, Report>
    where
        T: DeserializeOwned + Sync + Send,
    {
        match resp.json::<T>().await {
            Ok(j) => Ok(j),
            Err(e) => Err(e.into()),
        }
    }
}
