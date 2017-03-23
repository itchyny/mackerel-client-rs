//! Mackerel API client
use std::default;
use std::str::FromStr;
use std::convert::Into;
use hyper;
use hyper_native_tls;
use serde;
use serde_json;
use ::errors::*;

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
            api_base: "https://mackerel.io".to_string(),
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
        Client { api_key: api_key.to_string(), ..Default::default() }
    }

    /// Creates a new API client from API key and API base.
    pub fn new_with_api_base(api_key: &str, api_base: &str) -> Client {
        Client {
            api_key: api_key.to_string(),
            api_base: api_base.to_string(),
            ..Default::default()
        }
    }

    fn build_url(&self, path: &str, queries: Vec<(&str, Vec<&str>)>) -> hyper::Url {
        let url_str = self.api_base.clone() + path;
        let mut url = hyper::Url::from_str(&url_str).unwrap();
        for (name, values) in queries {
            for value in values {
                url.query_pairs_mut().append_pair(name, value);
            }
        }
        url
    }

    fn new_client(&self, scheme: &str) -> hyper::Client {
        match scheme {
            "https" => {
                let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
                let connector = hyper::net::HttpsConnector::new(ssl);
                hyper::Client::with_connector(connector)
            }
            "http" => hyper::Client::new(),
            _ => panic!("unknown scheme: {}", scheme),
        }
    }

    fn new_headers(&self) -> hyper::header::Headers {
        let mut headers = hyper::header::Headers::new();
        headers.set_raw("X-Api-Key", vec![self.api_key.clone().into_bytes()]);
        headers.set(hyper::header::ContentType::json());
        let url = hyper::Url::from_str(self.api_base.clone().as_str()).unwrap();
        if let (username, Some(password)) = (url.username(), url.password()) {
            if username != "" {
                headers.set(hyper::header::Authorization(hyper::header::Basic {
                    username: username.to_string(),
                    password: Some(password.to_string()),
                }))
            }
        }
        headers
    }

    /// Sends a request to the API.
    ///
    /// The entire response body is deserialized as `R`, converted by `converter`
    /// and returns `S`.
    pub fn request<P, B, R, F, S>(&self,
                                  method: hyper::method::Method,
                                  path: P,
                                  queries: Vec<(&str, Vec<&str>)>,
                                  body_opt: Option<B>,
                                  converter: F)
                                  -> Result<S>
        where P: AsRef<str>,
              B: serde::ser::Serialize,
              R: serde::de::Deserialize,
              F: FnOnce(R) -> S
    {
        let url = self.build_url(path.as_ref(), queries);
        let body_bytes = body_opt.map(|b| serde_json::to_vec(&b).unwrap()).unwrap_or(vec![]);
        let res = self.new_client(url.scheme())
            .request(method, url.clone())
            .headers(self.new_headers())
            .body(body_bytes.as_slice())
            .send()
            .chain_err(|| format!("request failed {}", url.clone()))?;
        if res.status != hyper::status::StatusCode::Ok {
            bail!(format!("got {}", res.status));
        }
        serde_json::from_reader(res)
            .map(converter)
            .chain_err(|| format!("JSON deserialization failed"))
    }
}
