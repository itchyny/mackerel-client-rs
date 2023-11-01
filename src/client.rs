//! Mackerel API client
use reqwest;
use serde;
use serde_json;
use typed_builder::TypedBuilder;
use url;

use crate::error::*;

/// An API client for Mackerel.
#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Client {
    api_key: String,
    #[builder(default = "https://api.mackerelio.com".to_string())]
    api_base: String,
    #[builder(default = format!("mackerel-client-rs/{}", env!("CARGO_PKG_VERSION")))]
    user_agent: String,
    #[builder(default = reqwest::Client::new(), setter(skip))]
    client: reqwest::Client,
}

impl Client {
    /// Creates a new API client from API key.
    pub fn new(api_key: impl AsRef<str>) -> Client {
        Self::builder().api_key(api_key.as_ref()).build()
    }

    fn build_url(&self, path: &str, queries: &[(&str, &str)]) -> url::Url {
        let mut url = url::Url::parse(&self.api_base)
            .unwrap_or_else(|err| panic!("{}: {}", err, self.api_base))
            .join(path)
            .unwrap();
        for (name, value) in queries {
            if !value.is_empty() {
                url.query_pairs_mut().append_pair(name, value);
            }
        }
        url
    }

    fn new_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "X-Api-Key",
            reqwest::header::HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(&self.user_agent).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers
    }

    pub(crate) async fn request<R, S>(
        &self,
        method: reqwest::Method,
        path: impl AsRef<str>,
        queries: &[(&str, &str)],
        body_opt: Option<impl serde::ser::Serialize>,
        converter: impl FnOnce(R) -> S,
    ) -> Result<S>
    where
        for<'de> R: serde::de::Deserialize<'de>,
    {
        let url = self.build_url(path.as_ref(), queries);
        let body_bytes = body_opt
            .map(|b| serde_json::to_vec(&b).unwrap())
            .unwrap_or_default();
        let response = {
            let request = self
                .client
                .request(method, url.clone())
                .headers(self.new_headers())
                .body(body_bytes);
            if url.username() != "" {
                request.basic_auth(url.username(), url.password())
            } else {
                request
            }
        }
        .send()
        .await
        .map_err(|e| Error::MsgError(format!("failed to send request: {}", e)))?;
        if !response.status().is_success() {
            return Err(self.api_error(response).await);
        }
        response
            .json::<R>()
            .await
            .map(converter)
            .map_err(|e| Error::MsgError(format!("JSON deserialization failed: {}", e)))
    }

    async fn api_error(&self, response: reqwest::Response) -> Error {
        let status = response.status();
        let message_opt =
            response
                .json::<serde_json::Value>()
                .await
                .ok()
                .and_then(|value: serde_json::Value| {
                    value
                        .get("error")
                        .map(|err| err.get("message").unwrap_or(err))
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                });
        Error::ApiError(status, message_opt.unwrap_or_default())
    }
}
