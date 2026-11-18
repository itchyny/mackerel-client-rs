use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use std::collections::HashMap;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::log::OrderDirection;

/// Parameters to search traces
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListTracesParams {
    pub service_name: String,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub from: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub to: DateTime<Utc>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_fingerprint: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<TraceStatusCode>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_latency_millis: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_latency_millis: Option<u64>,
    #[builder(
        default,
        setter(transform = |attributes: impl IntoIterator<Item = TraceAttributeFilter>| attributes
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<TraceAttributeFilter>,
    #[builder(
        default,
        setter(transform = |attributes: impl IntoIterator<Item = TraceAttributeFilter>| attributes
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_attributes: Vec<TraceAttributeFilter>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u8>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<TraceOrder>,
}

/// Status code to filter traces by
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum TraceStatusCode {
    Ok,
    Error,
}

/// A condition on an attribute of spans
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct TraceAttributeFilter {
    pub key: String,
    pub value: String,
    pub operator: TraceAttributeOperator,
    #[serde(rename = "type")]
    pub value_type: TraceAttributeValueType,
}

/// Comparison operator of a trace attribute filter
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceAttributeOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    StartsWith,
}

/// Value type of a trace attribute filter
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum TraceAttributeValueType {
    String,
    Int,
    Double,
    Bool,
}

/// Sort condition of traces
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct TraceOrder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<TraceOrderColumn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<OrderDirection>,
}

/// Column to sort traces by
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceOrderColumn {
    Latency,
    StartAt,
}

/// A trace in the search results
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct TraceSummary {
    pub trace_id: String,
    pub service_name: String,
    #[builder(default)]
    #[serde(default)]
    pub service_namespace: String,
    #[builder(default)]
    #[serde(default)]
    pub environment: String,
    pub title: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub trace_start_at: DateTime<Utc>,
    #[builder(setter(!into))]
    pub trace_latency_millis: u64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub service_start_at: DateTime<Utc>,
    #[builder(setter(!into))]
    pub service_latency_millis: u64,
}

/// A span of a trace
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct Span {
    pub trace_id: String,
    pub span_id: String,
    #[builder(default)]
    #[serde(default)]
    pub trace_state: String,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    pub name: String,
    pub kind: SpanKind,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    #[builder(default)]
    #[serde(default)]
    pub attributes: Vec<TraceAttribute>,
    #[builder(default, setter(!into))]
    #[serde(default)]
    pub dropped_attributes_count: u32,
    #[builder(default)]
    #[serde(default)]
    pub events: Vec<SpanEvent>,
    #[builder(default, setter(!into))]
    #[serde(default)]
    pub dropped_events_count: u32,
    #[builder(default)]
    #[serde(default)]
    pub links: Vec<SpanLink>,
    #[builder(default, setter(!into))]
    #[serde(default)]
    pub dropped_links_count: u32,
    pub status: SpanStatus,
    pub resource: SpanResource,
    pub scope: SpanScope,
}

/// Span kind
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum SpanKind {
    Unspecified,
    Internal,
    Server,
    Client,
    Producer,
    Consumer,
}

/// An attribute of a span, an event, a link, a resource or a scope
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct TraceAttribute {
    pub key: String,
    pub value: TraceAttributeValue,
}

/// A value of a trace attribute
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "valueType", rename_all = "lowercase")]
pub enum TraceAttributeValue {
    #[serde(rename_all = "camelCase")]
    String {
        string_value: String,
    },
    #[serde(rename_all = "camelCase")]
    Bool {
        bool_value: bool,
    },
    #[serde(rename_all = "camelCase")]
    Int {
        int_value: i64,
    },
    #[serde(rename_all = "camelCase")]
    Double {
        double_value: f64,
    },
    #[serde(rename_all = "camelCase")]
    Array {
        array_value: Vec<TraceAttributeValue>,
    },
    #[serde(rename_all = "camelCase")]
    Kvlist {
        kvlist_value: HashMap<String, TraceAttributeValue>,
    },
    #[serde(rename_all = "camelCase")]
    Bytes {
        bytes_value: String,
    },
    Empty,
}

/// An event of a span
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SpanEvent {
    pub time: DateTime<Utc>,
    pub name: String,
    #[builder(default)]
    #[serde(default)]
    pub attributes: Vec<TraceAttribute>,
    #[builder(default, setter(!into))]
    #[serde(default)]
    pub dropped_attributes_count: u32,
}

/// A link from a span to another span
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SpanLink {
    pub trace_id: String,
    pub span_id: String,
    #[builder(default)]
    #[serde(default)]
    pub trace_state: String,
    #[builder(default)]
    #[serde(default)]
    pub attributes: Vec<TraceAttribute>,
    #[builder(default, setter(!into))]
    #[serde(default)]
    pub dropped_attributes_count: u32,
}

/// Status of a span
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct SpanStatus {
    #[builder(default)]
    #[serde(default)]
    pub message: String,
    pub code: SpanStatusCode,
}

/// Status code of a span
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum SpanStatusCode {
    Unset,
    Ok,
    Error,
}

/// Resource of a span
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SpanResource {
    #[serde(default)]
    pub attributes: Vec<TraceAttribute>,
    #[builder(setter(!into))]
    #[serde(default)]
    pub dropped_attributes_count: u32,
}

/// Instrumentation scope of a span
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SpanScope {
    pub name: String,
    #[builder(default)]
    #[serde(default)]
    pub version: String,
    #[builder(default)]
    #[serde(default)]
    pub attributes: Vec<TraceAttribute>,
    #[builder(default, setter(!into))]
    #[serde(default)]
    pub dropped_attributes_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    #[test]
    fn test_list_traces_params_json() {
        let params = ListTracesParams::builder()
            .service_name("shoppingcart")
            .from(DateTime::from_timestamp(1718802000, 0).unwrap())
            .to(DateTime::from_timestamp(1718888400, 0).unwrap())
            .build();
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({
                "serviceName": "shoppingcart",
                "from": 1718802000,
                "to": 1718888400,
            }),
        );

        let params = ListTracesParams::builder()
            .service_name("shoppingcart")
            .service_namespace("shop")
            .from(DateTime::from_timestamp(1718802000, 0).unwrap())
            .to(DateTime::from_timestamp(1718888400, 0).unwrap())
            .environment("production")
            .trace_id("550e8400e29b41d4a716446655440000")
            .span_name("GET /api/users")
            .version("1.0.0")
            .issue_fingerprint("fingerprint")
            .status_code(TraceStatusCode::Error)
            .min_latency_millis(1000)
            .max_latency_millis(5000)
            .attributes([TraceAttributeFilter::builder()
                .key("http.status_code")
                .value("500")
                .operator(TraceAttributeOperator::Gte)
                .value_type(TraceAttributeValueType::Int)
                .build()])
            .resource_attributes([TraceAttributeFilter::builder()
                .key("host.name")
                .value("server")
                .operator(TraceAttributeOperator::StartsWith)
                .value_type(TraceAttributeValueType::String)
                .build()])
            .page(2)
            .per_page(20)
            .order(
                TraceOrder::builder()
                    .column(TraceOrderColumn::StartAt)
                    .direction(OrderDirection::Desc)
                    .build(),
            )
            .build();
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({
                "serviceName": "shoppingcart",
                "serviceNamespace": "shop",
                "from": 1718802000,
                "to": 1718888400,
                "environment": "production",
                "traceId": "550e8400e29b41d4a716446655440000",
                "spanName": "GET /api/users",
                "version": "1.0.0",
                "issueFingerprint": "fingerprint",
                "statusCode": "ERROR",
                "minLatencyMillis": 1000,
                "maxLatencyMillis": 5000,
                "attributes": [
                    {
                        "key": "http.status_code",
                        "value": "500",
                        "operator": "GTE",
                        "type": "int",
                    },
                ],
                "resourceAttributes": [
                    {
                        "key": "host.name",
                        "value": "server",
                        "operator": "STARTS_WITH",
                        "type": "string",
                    },
                ],
                "page": 2,
                "perPage": 20,
                "order": { "column": "START_AT", "direction": "DESC" },
            }),
        );
    }

    #[test]
    fn test_trace_summary_json() {
        let trace_summary = TraceSummary::builder()
            .trace_id("550e8400e29b41d4a716446655440000")
            .service_name("shoppingcart")
            .service_namespace("shop")
            .environment("production")
            .title("GET /api/users")
            .trace_start_at(DateTime::from_timestamp(1718802000, 0).unwrap())
            .trace_latency_millis(1234)
            .service_start_at(DateTime::from_timestamp(1718802100, 0).unwrap())
            .service_latency_millis(567)
            .build();
        let json = json!({
            "traceId": "550e8400e29b41d4a716446655440000",
            "serviceName": "shoppingcart",
            "serviceNamespace": "shop",
            "environment": "production",
            "title": "GET /api/users",
            "traceStartAt": 1718802000,
            "traceLatencyMillis": 1234,
            "serviceStartAt": 1718802100,
            "serviceLatencyMillis": 567,
        });
        assert_eq!(serde_json::to_value(&trace_summary).unwrap(), json);
        assert_eq!(trace_summary, serde_json::from_value(json).unwrap());
    }

    fn span_example() -> Span {
        Span::builder()
            .trace_id("0123456789abcdef0123456789abcdef")
            .span_id("0123456789abcdef")
            .trace_state("congo=xx,key=val")
            .parent_span_id("fedcba9876543210")
            .name("test-span")
            .kind(SpanKind::Internal)
            .start_time(DateTime::from_timestamp(1752069782, 0).unwrap())
            .end_time(DateTime::from_timestamp(1752069782, 500_000_000).unwrap())
            .attributes([TraceAttribute::builder()
                .key("http.route")
                .value(TraceAttributeValue::String {
                    string_value: "/".to_owned(),
                })
                .build()])
            .events([SpanEvent::builder()
                .time(DateTime::from_timestamp(1752069782, 0).unwrap())
                .name("event1")
                .build()])
            .links([SpanLink::builder()
                .trace_id("abcdef0123456789abcdef0123456789")
                .span_id("abcdefabcdef0102")
                .build()])
            .status(
                SpanStatus::builder()
                    .message("status message")
                    .code(SpanStatusCode::Ok)
                    .build(),
            )
            .resource(SpanResource::builder().build())
            .scope(
                SpanScope::builder()
                    .name("my-library")
                    .version("1.0.0")
                    .build(),
            )
            .build()
    }

    fn span_json_example() -> serde_json::Value {
        json!({
            "traceId": "0123456789abcdef0123456789abcdef",
            "spanId": "0123456789abcdef",
            "traceState": "congo=xx,key=val",
            "parentSpanId": "fedcba9876543210",
            "name": "test-span",
            "kind": "internal",
            "startTime": "2025-07-09T14:03:02Z",
            "endTime": "2025-07-09T14:03:02.500Z",
            "attributes": [
                {
                    "key": "http.route",
                    "value": { "valueType": "string", "stringValue": "/" },
                },
            ],
            "droppedAttributesCount": 0,
            "events": [
                {
                    "time": "2025-07-09T14:03:02Z",
                    "name": "event1",
                    "attributes": [],
                    "droppedAttributesCount": 0,
                },
            ],
            "droppedEventsCount": 0,
            "links": [
                {
                    "traceId": "abcdef0123456789abcdef0123456789",
                    "spanId": "abcdefabcdef0102",
                    "traceState": "",
                    "attributes": [],
                    "droppedAttributesCount": 0,
                },
            ],
            "droppedLinksCount": 0,
            "status": { "message": "status message", "code": "ok" },
            "resource": { "attributes": [], "droppedAttributesCount": 0 },
            "scope": {
                "name": "my-library",
                "version": "1.0.0",
                "attributes": [],
                "droppedAttributesCount": 0,
            },
        })
    }

    #[test]
    fn test_span_json() {
        assert_eq!(
            serde_json::to_value(span_example()).unwrap(),
            span_json_example()
        );
        assert_eq!(
            span_example(),
            serde_json::from_value(span_json_example()).unwrap()
        );
    }

    #[rstest]
    #[case(
        TraceAttributeValue::String { string_value: "success".to_owned() },
        json!({ "valueType": "string", "stringValue": "success" }),
    )]
    #[case(
        TraceAttributeValue::Bool { bool_value: true },
        json!({ "valueType": "bool", "boolValue": true }),
    )]
    #[case(
        TraceAttributeValue::Int { int_value: -10 },
        json!({ "valueType": "int", "intValue": -10 }),
    )]
    #[case(
        TraceAttributeValue::Double { double_value: 1.5 },
        json!({ "valueType": "double", "doubleValue": 1.5 }),
    )]
    #[case(
        TraceAttributeValue::Array {
            array_value: vec![
                TraceAttributeValue::Int { int_value: 10 },
                TraceAttributeValue::Int { int_value: 20 },
            ],
        },
        json!({
            "valueType": "array",
            "arrayValue": [
                { "valueType": "int", "intValue": 10 },
                { "valueType": "int", "intValue": 20 },
            ],
        }),
    )]
    #[case(
        TraceAttributeValue::Kvlist {
            kvlist_value: HashMap::from([(
                "en".to_owned(),
                TraceAttributeValue::String { string_value: "success".to_owned() },
            )]),
        },
        json!({
            "valueType": "kvlist",
            "kvlistValue": { "en": { "valueType": "string", "stringValue": "success" } },
        }),
    )]
    #[case(
        TraceAttributeValue::Bytes { bytes_value: "AQID".to_owned() },
        json!({ "valueType": "bytes", "bytesValue": "AQID" }),
    )]
    #[case(TraceAttributeValue::Empty, json!({ "valueType": "empty" }))]
    fn test_trace_attribute_value_json(
        #[case] value: TraceAttributeValue,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&value).unwrap(), json);
        assert_eq!(value, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(SpanKind::Unspecified, "unspecified")]
    #[case(SpanKind::Internal, "internal")]
    #[case(SpanKind::Server, "server")]
    #[case(SpanKind::Client, "client")]
    #[case(SpanKind::Producer, "producer")]
    #[case(SpanKind::Consumer, "consumer")]
    fn test_span_kind(#[case] span_kind: SpanKind, #[case] span_kind_str: &str) {
        assert_eq!(span_kind.to_string(), span_kind_str);
        assert_eq!(span_kind, span_kind_str.parse().unwrap());
    }
}

impl Client {
    /// Searches traces.
    ///
    /// The range between `from` and `to` must be 30 minutes or less.
    ///
    /// See <https://mackerel.io/api-docs/entry/traces#list>.
    pub async fn list_traces(
        &self,
        params: impl Borrow<ListTracesParams>,
    ) -> Result<(Vec<TraceSummary>, bool)> {
        self.request(
            Method::POST,
            "/api/v0/traces",
            query_params![],
            request_body!(params.borrow()),
            response_body! { results: Vec<TraceSummary>, hasNextPage: bool },
        )
        .await
    }

    /// Gets the spans of a trace.
    ///
    /// See <https://mackerel.io/api-docs/entry/traces#get>.
    pub async fn get_trace(&self, trace_id: impl AsRef<str>) -> Result<Vec<Span>> {
        self.request(
            Method::GET,
            format!("/api/v0/traces/{}", trace_id.as_ref()),
            query_params![],
            request_body![],
            response_body! { spans: Vec<Span> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use crate::tests::*;
    use crate::trace::*;

    #[tokio::test]
    async fn list_traces() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/traces",
            request = json!({
                "serviceName": "shoppingcart",
                "from": 1718802000,
                "to": 1718888400,
                "statusCode": "ERROR",
            }),
            response = json!({
                "results": [{
                    "traceId": "550e8400e29b41d4a716446655440000",
                    "serviceName": "shoppingcart",
                    "serviceNamespace": "",
                    "environment": "",
                    "title": "GET /api/users",
                    "traceStartAt": 1718802000,
                    "traceLatencyMillis": 1234,
                    "serviceStartAt": 1718802100,
                    "serviceLatencyMillis": 567,
                }],
                "hasNextPage": true,
            }),
        };
        let params = ListTracesParams::builder()
            .service_name("shoppingcart")
            .from(DateTime::from_timestamp(1718802000, 0).unwrap())
            .to(DateTime::from_timestamp(1718888400, 0).unwrap())
            .status_code(TraceStatusCode::Error)
            .build();
        let expected = Ok((
            vec![TraceSummary::builder()
                .trace_id("550e8400e29b41d4a716446655440000")
                .service_name("shoppingcart")
                .title("GET /api/users")
                .trace_start_at(DateTime::from_timestamp(1718802000, 0).unwrap())
                .trace_latency_millis(1234)
                .service_start_at(DateTime::from_timestamp(1718802100, 0).unwrap())
                .service_latency_millis(567)
                .build()],
            true,
        ));
        assert_eq!(test_client!(server).list_traces(&params).await, expected);
        assert_eq!(test_client!(server).list_traces(params).await, expected);
    }

    #[tokio::test]
    async fn get_trace() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/traces/0123456789abcdef0123456789abcdef",
            response = json!({
                "spans": [{
                    "traceId": "0123456789abcdef0123456789abcdef",
                    "spanId": "0123456789abcdef",
                    "traceState": "",
                    "name": "root",
                    "kind": "server",
                    "startTime": "2025-07-09T14:03:02.000Z",
                    "endTime": "2025-07-09T14:03:03.000Z",
                    "attributes": [],
                    "droppedAttributesCount": 0,
                    "events": [],
                    "droppedEventsCount": 0,
                    "links": [],
                    "droppedLinksCount": 0,
                    "status": { "message": "", "code": "unset" },
                    "resource": { "attributes": [], "droppedAttributesCount": 0 },
                    "scope": { "name": "", "version": "", "attributes": [], "droppedAttributesCount": 0 },
                }],
            }),
        };
        assert_eq!(
            test_client!(server)
                .get_trace("0123456789abcdef0123456789abcdef")
                .await,
            Ok(vec![Span::builder()
                .trace_id("0123456789abcdef0123456789abcdef")
                .span_id("0123456789abcdef")
                .name("root")
                .kind(SpanKind::Server)
                .start_time(DateTime::from_timestamp(1752069782, 0).unwrap())
                .end_time(DateTime::from_timestamp(1752069783, 0).unwrap())
                .status(SpanStatus::builder().code(SpanStatusCode::Unset).build())
                .resource(SpanResource::builder().build())
                .scope(SpanScope::builder().name("").build())
                .build()]),
        );
    }
}
