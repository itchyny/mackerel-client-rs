//! Mackerel API client
use crate::error::*;
use reqwest;
use serde;
use serde_json;
use typed_builder::TypedBuilder;
use url;

/// Represents an API client for Mackerel.
#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Client {
    api_key: String,
    #[builder(default = "https://api.mackerelio.com".to_string())]
    api_base: String,
    #[builder(default = format!("mackerel-client-rs/{}", env!("CARGO_PKG_VERSION")))]
    user_agent: String,
}

/// Empty body to avoid type ambiguity.
pub fn empty_body() -> Option<()> {
    None
}

impl Client {
    /// Creates a new API client from API key.
    pub fn new(api_key: impl Into<String>) -> Client {
        Self::builder().api_key(api_key).build()
    }

    fn build_url(&self, path: &str, queries: Vec<(&str, Vec<&str>)>) -> url::Url {
        let url_str = self.api_base.clone() + path;
        let mut url = url::Url::parse(url_str.as_str()).unwrap();
        for (name, values) in queries {
            for value in values {
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

    /// Sends a request to the API.
    ///
    /// The entire response body is deserialized as `R`, converted by `converter`
    /// and returns `S`.
    pub(crate) async fn request<R, S>(
        &self,
        method: reqwest::Method,
        path: impl AsRef<str>,
        queries: Vec<(&str, Vec<&str>)>,
        body_opt: Option<impl serde::ser::Serialize>,
        converter: impl FnOnce(R) -> S,
    ) -> Result<S>
    where
        for<'de> R: serde::de::Deserialize<'de>,
    {
        let client = reqwest::Client::new();
        let url = self.build_url(path.as_ref(), queries);
        let body_bytes = body_opt
            .map(|b| serde_json::to_vec(&b).unwrap())
            .unwrap_or_default();
        let response = {
            let request = client
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
        .map_err(|e| format!("failed to send request: {}", e))?;
        if !response.status().is_success() {
            return Err(self.api_error(response).await);
        }
        response
            .json::<R>()
            .await
            .map(converter)
            .map_err(|e| format!("JSON deserialization failed: {}", e).into())
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
                        .and_then(|val| val.as_str().map(|s| s.to_string()))
                });
        Error::ApiError(status, message_opt.unwrap_or("".to_string()))
    }
}
