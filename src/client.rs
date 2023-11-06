//! Mackerel API client
use http::{header::*, Method};
use reqwest;
use serde;
use serde_json;
use typed_builder::TypedBuilder;
use url::Url;

use crate::error::*;

/// An API client for Mackerel.
#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Client {
    #[builder(
        setter(transform = |s: impl AsRef<str>| HeaderValue::from_str(s.as_ref())
            .map(|mut header_value| { header_value.set_sensitive(true); header_value })
            .unwrap_or_else(|err| panic!("invalid api_key: {}", err))),
    )]
    api_key: HeaderValue,

    #[builder(
        default = "https://api.mackerelio.com".try_into().unwrap(),
        setter(transform = |s: impl AsRef<str>| Url::parse(s.as_ref())
            .unwrap_or_else(|err| panic!("invalid api_base ({:?}): {}", s.as_ref(), err))),
    )]
    api_base: Url,

    #[builder(
        default = format!("mackerel-client-rs/{}", env!("CARGO_PKG_VERSION")).try_into().unwrap(),
        setter(transform = |s: impl AsRef<str>| HeaderValue::from_str(s.as_ref())
            .unwrap_or_else(|err| panic!("invalid user_agent ({:?}): {}", s.as_ref(), err))),
    )]
    user_agent: HeaderValue,

    #[builder(default = reqwest::Client::new(), setter(skip))]
    client: reqwest::Client,
}

impl Client {
    /// Creates a new API client from API key.
    /// ```rust
    /// use mackerel_client::Client;
    ///
    /// let client = Client::new("<Mackerel-API-KEY>");
    /// ```
    /// If you want to configure the API base, use [`Client::builder()`].
    /// ```rust
    /// use mackerel_client::Client;
    ///
    /// let client = Client::builder()
    ///     .api_key("<Mackerel-API-KEY>")
    ///     .api_base("https://api.mackerelio.com")
    ///     .build();
    /// ```
    pub fn new(api_key: impl AsRef<str>) -> Client {
        Self::builder().api_key(api_key.as_ref()).build()
    }

    fn build_url(&self, path: &str, query_params: &[(&str, impl AsRef<str>)]) -> Url {
        let mut url = self.api_base.join(path).unwrap();
        if !query_params.is_empty() {
            url.query_pairs_mut().extend_pairs(
                query_params
                    .iter()
                    .filter(|(_, value)| !value.as_ref().is_empty()),
            );
        }
        url
    }

    pub(crate) async fn request<R, S>(
        &self,
        method: Method,
        path: impl AsRef<str>,
        query_params: &[(&str, impl AsRef<str>)],
        request_body_opt: Option<impl serde::ser::Serialize>,
        converter: impl FnOnce(R) -> S,
    ) -> Result<S>
    where
        for<'de> R: serde::de::Deserialize<'de>,
    {
        let url = self.build_url(path.as_ref(), query_params);
        let request_body_bytes = request_body_opt
            .map(|b| serde_json::to_vec(&b).unwrap())
            .unwrap_or_default();
        let response = {
            let request = self
                .client
                .request(method, url.clone())
                .headers(HeaderMap::from_iter([
                    (HeaderName::from_static("x-api-key"), self.api_key.clone()),
                    (USER_AGENT, self.user_agent.clone()),
                    (CONTENT_TYPE, HeaderValue::from_static("application/json")),
                ]))
                .body(request_body_bytes);
            if url.username() != "" {
                request.basic_auth(url.username(), url.password())
            } else {
                request
            }
        }
        .send()
        .await
        .map_err(|err| Error::MsgError(format!("failed to send request: {}", err)))?;
        if !response.status().is_success() {
            return Err(self.api_error(response).await);
        }
        response
            .json::<R>()
            .await
            .map(converter)
            .map_err(|err| Error::MsgError(format!("JSON deserialization failed: {}", err)))
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

macro_rules! format_url {
    ( $format:expr, $( $args:expr ),+ $(,)? ) => {
        format!($format, $( $args.into() ),+)
    };
}
pub(crate) use format_url;

macro_rules! query_params {
    [] => {
        &[] as &[(&str, &str); 0]
    };
    { $( $field:ident = $value:expr ),* $(,)? } => {{
        &[
            $( (stringify!($field), &$value) ),*
        ]
    }};
}
pub(crate) use query_params;

macro_rules! request_body {
    [] => {
        None::<()>
    };
    ( $value:expr $(,)? ) => {
        Some($value)
    };
    { $( $field:ident: $type:ty = $value:expr ),+ $(,)? } => {{
        #[allow(non_snake_case)]
        #[derive(::serde_derive::Serialize)]
        struct Request { $( $field: $type ),* }
        #[allow(clippy::redundant_field_names)]
        Some(Request { $( $field: $value ),* })
    }};
}
pub(crate) use request_body;

macro_rules! response_body {
    () => {
        |_: ::serde_json::Value| ()
    };
    (..) => {
        |response| response
    };
    { $( $field:ident: $type:ty ),+ $(,)? } => {{
        #[allow(non_snake_case)]
        #[derive(::serde_derive::Deserialize)]
        struct Response { $( $field: $type ),* }
        |response: Response| ($( response.$field ),*)
    }};
}
pub(crate) use response_body;

#[cfg(test)]
mod client_tests {
    use http::{Method, StatusCode};

    use crate::client::*;
    use crate::error::Error;
    use crate::tests::*;

    impl Client {
        async fn get(&self) -> Result<bool> {
            self.request(
                Method::GET,
                "/api/v0/test",
                query_params![],
                request_body![],
                response_body!(..),
            )
            .await
        }

        async fn post(&self) -> Result<bool> {
            self.request(
                Method::POST,
                "/api/v0/test",
                query_params! {
                    param1 = "value1",
                    param2 = "value2",
                    param3 = "value3",
                },
                request_body! {
                    message: String = "request body".to_owned(),
                },
                response_body! {
                    success: bool,
                },
            )
            .await
        }
    }

    #[async_std::test]
    async fn success() {
        {
            let server = test_server! {
                method = GET,
                path = "/api/v0/test",
                status_code = 200,
                response = json!(true),
            };
            assert_eq!(test_client!(server).get().await, Ok(true));
        }
        {
            let server = test_server! {
                method = POST,
                path = "/api/v0/test",
                query_params = "param1=value1&param2=value2&param3=value3",
                status_code = 201,
                request = json!({ "message": "request body" }),
                response = json!({ "success": true }),
            };
            assert_eq!(test_client!(server).post().await, Ok(true));
        }
    }

    #[async_std::test]
    async fn error() {
        {
            let server = test_server! {
                method = GET,
                path = "/api/v0/test",
                status_code = 400,
                response = json!({
                    "error": "This is an error message.",
                }),
            };
            assert_eq!(
                test_client!(server).get().await,
                Err(Error::ApiError(
                    StatusCode::BAD_REQUEST,
                    "This is an error message.".to_owned()
                )),
            );
        }
        {
            let server = test_server! {
                method = GET,
                path = "/api/v0/test",
                status_code = 500,
                response = json!({
                    "error": {
                        "message": "This is an error message.",
                    },
                }),
            };
            assert_eq!(
                test_client!(server).get().await,
                Err(Error::ApiError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "This is an error message.".to_owned()
                )),
            );
        }
    }
}
