use http::StatusCode;
use httptest::ServerPool;
use serde_json::Value;

pub(crate) static TEST_SERVER_POOL: ServerPool = ServerPool::new(3);

#[derive(Default)]
pub(crate) struct TestServerConfig {
    pub method: &'static str,
    pub path: &'static str,
    pub query_params: &'static str,
    pub request: Value,
    pub status_code: StatusCode,
    pub response: Value,
}

pub(crate) const GET: &str = "GET";
pub(crate) const POST: &str = "POST";
pub(crate) const PUT: &str = "PUT";
pub(crate) const DELETE: &str = "DELETE";

macro_rules! test_server {
    ($( $field:ident = $value:expr ),* $(,)? ) => {{
        use ::httptest::{all_of, matchers::*, responders, Expectation};
        use ::serde_json::Value;
        use ::std::{boxed::Box, vec::Vec};
        #[allow(unused_imports)]
        use ::serde_json::json;
        let _ = pretty_env_logger::try_init();
        #[allow(clippy::needless_update)]
        let config = TestServerConfig {
            $( $field: $value.try_into().unwrap_or_else(|err| {
                panic!("failed to convert {:?} into {}: {}", $value, stringify!($field), err);
            }), )*
            ..TestServerConfig::default()
        };
        let server = $crate::tests::TEST_SERVER_POOL.get_server();
        server.expect(
            Expectation::matching(all_of![
                request::method(config.method),
                request::headers(all_of![
                    contains(key(lowercase("x-api-key"))),
                    contains((lowercase("user-agent"), matches("^mackerel-client-rs/"))),
                    contains((lowercase("content-type"), "application/json")),
                ]),
                request::path(config.path),
                request::query(url_decoded(all_of(
                    config.query_params
                        .split_terminator('&')
                        .map(|param| {
                            Box::new(contains(param.split_once('=').unwrap_or((param, "")))) as _
                        })
                        .chain(::std::iter::once(
                            Box::new(len(eq(config.query_params.split_terminator('&').count()))) as _
                        ))
                        .collect::<Vec<_>>(),
                ))),
                request::body(
                    if config.request == Value::Null {
                        all_of![""]
                    } else {
                        all_of![json_decoded(eq(config.request))]
                    },
                ),
            ])
            .times(1..)
            .respond_with(
                responders::status_code(config.status_code.as_u16())
                    .append_header("Content-Type", "application/json")
                    .body(::serde_json::to_string(&config.response).unwrap()),
            ),
        );
        server
    }};
}
pub(crate) use test_server;

macro_rules! test_client {
    ($server:expr) => {
        $crate::client::Client::builder()
            .api_key("")
            .api_base($server.url_str("/"))
            .build()
    };
}
pub(crate) use test_client;
