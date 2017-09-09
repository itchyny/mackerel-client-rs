//! Mackerel API client
use std::default;
use std::str::FromStr;
use std::convert::Into;
use url::Url;
use tokio_core;
use hyper;
use hyper_tls;
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

    fn build_uri(&self, path: &str, queries: Vec<(&str, Vec<&str>)>) -> hyper::Uri {
        let url_str = self.api_base.clone() + path;
        let mut url = Url::parse(url_str.as_str()).unwrap();
        for (name, values) in queries {
            for value in values {
                url.query_pairs_mut().append_pair(name, value);
            }
        }
        hyper::Uri::from_str(url.as_str()).unwrap()
    }

    fn new_client(&self, scheme: &str) -> hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>> {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        match scheme {
            "https" => {
                hyper::Client::configure()
                    .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
                    .build(&handle)
            }
            // "http" => hyper::Client::new(&handle),
            "http" => panic!("http not supported"),
            _ => panic!("unknown scheme: {}", scheme),
        }
    }

    fn set_headers(&self, headers: &mut hyper::header::Headers) {
        headers.set_raw("X-Api-Key", vec![self.api_key.clone().into_bytes()]);
        headers.set(hyper::header::ContentType::json());
        let url = Url::from_str(self.api_base.clone().as_str()).unwrap();
        if let (username, Some(password)) = (url.username(), url.password()) {
            if username != "" {
                headers.set(hyper::header::Authorization(hyper::header::Basic {
                    username: username.to_string(),
                    password: Some(password.to_string()),
                }))
            }
        };
    }

    /// Sends a request to the API.
    ///
    /// The entire response body is deserialized as `R`, converted by `converter`
    /// and returns `S`.
    pub fn request<P, B, R, F, S>(&self,
                                  method: hyper::Method,
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
        let uri = self.build_uri(path.as_ref(), queries);
        let mut req = hyper::Request::new(method, uri);
        self.set_headers(req.headers_mut());
        req.set_body(body_opt.map(|b| serde_json::to_vec(&b).unwrap()).unwrap_or(vec![]).as_slice());
        let response = self.new_client(uri.scheme().unwrap())
            .request(req)
            .send()
            .chain_err(|| format!("request failed {}", uri.clone()))?;
        if response.status != hyper::StatusCode::Ok {
            bail!(self.api_error(response))
        }
        serde_json::from_reader(response)
            .map(converter)
            .chain_err(|| format!("JSON deserialization failed"))
    }

    fn api_error(&self, response: hyper::client::Response) -> ErrorKind {
        let status = response.status;
        let message_opt = serde_json::from_reader(response)
            .ok()
            .and_then(|value: serde_json::Value| {
                value.get("error")
                    .map(|err| err.get("message").unwrap_or(err))
                    .and_then(|val| val.as_str().map(|s| s.to_string()))
            });
        ErrorKind::ApiError(status, message_opt.unwrap_or("".to_string()))
    }
}
