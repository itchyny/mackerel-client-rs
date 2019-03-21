//! Mackerel API client
use std::default;
use std::str::FromStr;
use std::convert::Into;
use url;
use reqwest;
use serde;
use serde_json;
use errors::*;

/// Represents an API client for Mackerel.
#[derive(Debug)]
pub struct Client {
    api_key: String,
    api_base: String,
    user_agent: String,
}

impl default::Default for Client {
    fn default() -> Client {
        Client {
            api_key: "".to_string(),
            api_base: "https://api.mackerelio.com".to_string(),
            user_agent: "mackerel-client-rs".to_string(),
        }
    }
}

/// Empty body to avoid type ambiguity.
pub fn empty_body() -> Option<()> {
    None
}

impl Client {
    /// Creates a new API client from API key.
    pub fn new(api_key: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
            ..Default::default()
        }
    }

    /// Creates a new API client from API key and API base.
    pub fn new_with_api_base(api_key: &str, api_base: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
            api_base: api_base.to_string(),
            ..Default::default()
        }
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

    fn new_headers(&self) -> reqwest::header::Headers {
        let mut headers = reqwest::header::Headers::new();
        headers.set_raw("X-Api-Key", vec![self.api_key.clone().into_bytes()]);
        headers.set(reqwest::header::ContentType::json());
        let url = url::Url::from_str(self.api_base.clone().as_str()).unwrap();
        if let (username, Some(password)) = (url.username(), url.password()) {
            if username != "" {
                headers.set(reqwest::header::Authorization(reqwest::header::Basic {
                    username: username.to_string(),
                    password: Some(password.to_string()),
                }))
            }
        };
        headers
    }

    /// Sends a request to the API.
    ///
    /// The entire response body is deserialized as `R`, converted by `converter`
    /// and returns `S`.
    pub fn request<P, B, R, F, S>(
        &self,
        method: reqwest::Method,
        path: P,
        queries: Vec<(&str, Vec<&str>)>,
        body_opt: Option<B>,
        converter: F,
    ) -> Result<S>
    where
        P: AsRef<str>,
        B: serde::ser::Serialize,
        for<'de> R: serde::de::Deserialize<'de>,
        F: FnOnce(R) -> S,
    {
        let client = reqwest::Client::new().chain_err(|| format!("failed to create a client"))?;
        let url = self.build_url(path.as_ref(), queries);
        let body_bytes = body_opt
            .map(|b| serde_json::to_vec(&b).unwrap())
            .unwrap_or(vec![]);
        let response = client
            .request(method, url)
            .and_then(|mut req| req.headers(self.new_headers()).body(body_bytes).send())
            .map_err(|e| format!("failed to send request: {}", e))?;
        if !response.status().is_success() {
            bail!(self.api_error(response))
        }
        serde_json::from_reader(response)
            .map(converter)
            .chain_err(|| format!("JSON deserialization failed"))
    }

    fn api_error(&self, response: reqwest::Response) -> ErrorKind {
        let status = response.status();
        let message_opt = serde_json::from_reader(response)
            .ok()
            .and_then(|value: serde_json::Value| {
                value
                    .get("error")
                    .map(|err| err.get("message").unwrap_or(err))
                    .and_then(|val| val.as_str().map(|s| s.to_string()))
            });
        ErrorKind::ApiError(status, message_opt.unwrap_or("".to_string()))
    }
}
