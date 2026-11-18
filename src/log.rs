use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;

/// Parameters to search logs
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLogsParams {
    pub service_name: String,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub from: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub to: DateTime<Utc>,
    #[builder(
        default,
        setter(transform = |keywords: impl IntoIterator<Item = impl Into<String>>| keywords
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[builder(
        default,
        setter(transform = |severities: impl IntoIterator<Item = LogSeverity>| severities
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub severities: Vec<LogSeverity>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[builder(
        default,
        setter(transform = |body_attributes: impl IntoIterator<Item = LogBodyAttributeFilter>| body_attributes
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub body_attributes: Vec<LogBodyAttributeFilter>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<LogOrder>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_cursor: Option<LogPageCursor>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u16>,
}

/// Log severity
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum LogSeverity {
    Unspecified,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

/// A condition on a field of a structured (JSON) log body
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct LogBodyAttributeFilter {
    #[builder(
        setter(transform = |key_path: impl IntoIterator<Item = impl Into<String>>| key_path
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    pub key_path: Vec<String>,
    pub value: String,
    pub operator: LogBodyAttributeOperator,
    #[serde(rename = "type")]
    pub value_type: LogBodyAttributeValueType,
}

/// Comparison operator of a log body attribute filter
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum LogBodyAttributeOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    StartsWith,
}

/// Value type of a log body attribute filter
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum LogBodyAttributeValueType {
    String,
    Int,
    Double,
    Bool,
}

/// Sort condition of logs
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct LogOrder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<LogOrderColumn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<OrderDirection>,
}

/// Column to sort logs by
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum LogOrderColumn {
    Timestamp,
}

/// Sort direction
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Starting point of log paging, which is excluded from the results
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct LogPageCursor {
    pub direction: LogPageCursorDirection,
    pub cursor: String,
}

/// Direction to read logs from a cursor
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum LogPageCursorDirection {
    Next,
    Previous,
}

/// A log record
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub cursor: String,
    pub timestamp: DateTime<Utc>,
    pub severity: LogSeverity,
    #[builder(setter(!into))]
    pub severity_number: u8,
    pub body: String,
    #[builder(default, setter(strip_option))]
    pub trace_id: Option<String>,
    #[builder(default, setter(strip_option))]
    pub span_id: Option<String>,
    pub service_name: String,
    #[builder(default, setter(strip_option))]
    pub service_namespace: Option<String>,
    #[builder(default)]
    #[serde(default)]
    pub attributes: Vec<LogAttribute>,
    #[builder(default)]
    #[serde(default)]
    pub resource_attributes: Vec<LogAttribute>,
    #[builder(default)]
    #[serde(default)]
    pub scope_attributes: Vec<LogAttribute>,
}

/// An attribute of a log record, its resource or its scope
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct LogAttribute {
    pub key: String,
    pub value: LogAttributeValue,
}

/// A value of a log attribute
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct LogAttributeValue {
    pub value_type: String,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

impl From<&str> for LogAttributeValue {
    fn from(value: &str) -> Self {
        Self::builder()
            .value_type("string")
            .string_value(value)
            .build()
    }
}

/// Paging information of logs
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct LogPagination {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    #[builder(default, setter(strip_option))]
    pub start_cursor: Option<String>,
    #[builder(default, setter(strip_option))]
    pub end_cursor: Option<String>,
}

/// A saved log search entity
pub type SavedLogSearch = Entity<SavedLogSearchValue>;

/// A saved log search id
pub type SavedLogSearchId = Id<SavedLogSearchValue>;

/// A saved log search value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SavedLogSearchValue {
    pub name: String,
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    pub definition: SavedLogSearchDefinition,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Search condition of a saved log search
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct SavedLogSearchDefinition {
    pub service_name: String,
    #[builder(default, setter(strip_option))]
    pub service_namespace: Option<String>,
    #[builder(default)]
    #[serde(default)]
    pub keywords: Vec<String>,
    #[builder(default)]
    #[serde(default)]
    pub severities: Vec<LogSeverity>,
    #[builder(default, setter(strip_option))]
    pub trace_id: Option<String>,
    #[builder(default)]
    #[serde(default)]
    pub body_attributes: Vec<LogBodyAttributeFilter>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    #[test]
    fn test_list_logs_params_json() {
        let params = ListLogsParams::builder()
            .service_name("shoppingcart")
            .from(DateTime::from_timestamp(1788220800, 0).unwrap())
            .to(DateTime::from_timestamp(1788739200, 0).unwrap())
            .build();
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({
                "serviceName": "shoppingcart",
                "from": 1788220800,
                "to": 1788739200,
            }),
        );

        let params = ListLogsParams::builder()
            .service_name("shoppingcart")
            .service_namespace("shop")
            .from(DateTime::from_timestamp(1788220800, 0).unwrap())
            .to(DateTime::from_timestamp(1788739200, 0).unwrap())
            .keywords(["error"])
            .severities([LogSeverity::Error, LogSeverity::Warn])
            .trace_id("550e8400e29b41d4a716446655440000")
            .body_attributes([
                LogBodyAttributeFilter::builder()
                    .key_path(["status_code"])
                    .value("500")
                    .operator(LogBodyAttributeOperator::Eq)
                    .value_type(LogBodyAttributeValueType::Int)
                    .build(),
                LogBodyAttributeFilter::builder()
                    .key_path(["req", "method"])
                    .value("GET")
                    .operator(LogBodyAttributeOperator::StartsWith)
                    .value_type(LogBodyAttributeValueType::String)
                    .build(),
            ])
            .order(
                LogOrder::builder()
                    .column(LogOrderColumn::Timestamp)
                    .direction(OrderDirection::Desc)
                    .build(),
            )
            .page_cursor(
                LogPageCursor::builder()
                    .direction(LogPageCursorDirection::Next)
                    .cursor("cursor-string")
                    .build(),
            )
            .per_page(100)
            .build();
        assert_eq!(
            serde_json::to_value(&params).unwrap(),
            json!({
                "serviceName": "shoppingcart",
                "serviceNamespace": "shop",
                "from": 1788220800,
                "to": 1788739200,
                "keywords": ["error"],
                "severities": ["ERROR", "WARN"],
                "traceId": "550e8400e29b41d4a716446655440000",
                "bodyAttributes": [
                    {
                        "keyPath": ["status_code"],
                        "value": "500",
                        "operator": "EQ",
                        "type": "int",
                    },
                    {
                        "keyPath": ["req", "method"],
                        "value": "GET",
                        "operator": "STARTS_WITH",
                        "type": "string",
                    },
                ],
                "order": { "column": "TIMESTAMP", "direction": "DESC" },
                "pageCursor": { "direction": "NEXT", "cursor": "cursor-string" },
                "perPage": 100,
            }),
        );
    }

    fn log_entry_example() -> LogEntry {
        LogEntry::builder()
            .cursor("cursor1")
            .timestamp(DateTime::from_timestamp(1788784496, 789123456).unwrap())
            .severity(LogSeverity::Error)
            .severity_number(17)
            .body("internal server error")
            .trace_id("550e8400e29b41d4a716446655440000")
            .span_id("a716446655440000")
            .service_name("shoppingcart")
            .service_namespace("shop")
            .attributes([LogAttribute::builder()
                .key("http.status_code")
                .value("500")
                .build()])
            .resource_attributes([LogAttribute::builder()
                .key("host.name")
                .value("host0")
                .build()])
            .build()
    }

    fn log_entry_json_example() -> serde_json::Value {
        json!({
            "cursor": "cursor1",
            "timestamp": "2026-09-07T12:34:56.789123456Z",
            "severity": "ERROR",
            "severityNumber": 17,
            "body": "internal server error",
            "traceId": "550e8400e29b41d4a716446655440000",
            "spanId": "a716446655440000",
            "serviceName": "shoppingcart",
            "serviceNamespace": "shop",
            "attributes": [
                {
                    "key": "http.status_code",
                    "value": { "valueType": "string", "stringValue": "500" },
                },
            ],
            "resourceAttributes": [
                {
                    "key": "host.name",
                    "value": { "valueType": "string", "stringValue": "host0" },
                },
            ],
            "scopeAttributes": [],
        })
    }

    #[test]
    fn test_log_entry_json() {
        assert_eq!(
            serde_json::to_value(log_entry_example()).unwrap(),
            log_entry_json_example(),
        );
        assert_eq!(
            log_entry_example(),
            serde_json::from_value(log_entry_json_example()).unwrap(),
        );
    }

    #[test]
    fn test_log_entry_json_nulls() {
        let log_entry: LogEntry = serde_json::from_value(json!({
            "cursor": "cursor2",
            "timestamp": "2026-09-07T12:34:56Z",
            "severity": "INFO",
            "severityNumber": 9,
            "body": "ok",
            "traceId": null,
            "spanId": null,
            "serviceName": "shoppingcart",
            "serviceNamespace": null,
            "attributes": [],
            "resourceAttributes": [],
            "scopeAttributes": [],
        }))
        .unwrap();
        assert_eq!(
            log_entry,
            LogEntry::builder()
                .cursor("cursor2")
                .timestamp(DateTime::from_timestamp(1788784496, 0).unwrap())
                .severity(LogSeverity::Info)
                .severity_number(9)
                .body("ok")
                .service_name("shoppingcart")
                .build(),
        );
    }

    #[rstest]
    #[case(LogSeverity::Unspecified, "UNSPECIFIED")]
    #[case(LogSeverity::Trace, "TRACE")]
    #[case(LogSeverity::Debug, "DEBUG")]
    #[case(LogSeverity::Info, "INFO")]
    #[case(LogSeverity::Warn, "WARN")]
    #[case(LogSeverity::Error, "ERROR")]
    #[case(LogSeverity::Fatal, "FATAL")]
    fn test_log_severity(#[case] log_severity: LogSeverity, #[case] log_severity_str: &str) {
        assert_eq!(log_severity.to_string(), log_severity_str);
        assert_eq!(log_severity, log_severity_str.parse().unwrap());
    }

    #[rstest]
    #[case(LogBodyAttributeOperator::Eq, "EQ")]
    #[case(LogBodyAttributeOperator::Neq, "NEQ")]
    #[case(LogBodyAttributeOperator::Gt, "GT")]
    #[case(LogBodyAttributeOperator::Gte, "GTE")]
    #[case(LogBodyAttributeOperator::Lt, "LT")]
    #[case(LogBodyAttributeOperator::Lte, "LTE")]
    #[case(LogBodyAttributeOperator::StartsWith, "STARTS_WITH")]
    fn test_log_body_attribute_operator(
        #[case] operator: LogBodyAttributeOperator,
        #[case] operator_str: &str,
    ) {
        assert_eq!(operator.to_string(), operator_str);
        assert_eq!(operator, operator_str.parse().unwrap());
    }
}

impl Client {
    /// Searches logs.
    ///
    /// See <https://mackerel.io/api-docs/entry/logs#list>.
    pub async fn list_logs(
        &self,
        params: impl Borrow<ListLogsParams>,
    ) -> Result<(Vec<LogEntry>, LogPagination)> {
        self.request(
            Method::POST,
            "/api/v0/logs",
            query_params![],
            request_body!(params.borrow()),
            response_body! { results: Vec<LogEntry>, pagination: LogPagination },
        )
        .await
    }

    /// Fetches all the saved log searches.
    ///
    /// See <https://mackerel.io/api-docs/entry/logs#list-saved-searches>.
    pub async fn list_saved_log_searches(&self) -> Result<Vec<SavedLogSearch>> {
        self.request(
            Method::GET,
            "/api/v0/saved-log-searches",
            query_params![],
            request_body![],
            response_body! { results: Vec<SavedLogSearch> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use crate::log::*;
    use crate::tests::*;

    #[tokio::test]
    async fn list_logs() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/logs",
            request = json!({
                "serviceName": "shoppingcart",
                "from": 1788220800,
                "to": 1788739200,
                "keywords": ["error"],
            }),
            response = json!({
                "results": [{
                    "cursor": "cursor1",
                    "timestamp": "2026-09-07T12:34:56Z",
                    "severity": "ERROR",
                    "severityNumber": 17,
                    "body": "internal server error",
                    "traceId": null,
                    "spanId": null,
                    "serviceName": "shoppingcart",
                    "serviceNamespace": null,
                    "attributes": [],
                    "resourceAttributes": [],
                    "scopeAttributes": [],
                }],
                "pagination": {
                    "hasNextPage": true,
                    "hasPreviousPage": false,
                    "startCursor": "cursor1",
                    "endCursor": "cursor1",
                },
            }),
        };
        let params = ListLogsParams::builder()
            .service_name("shoppingcart")
            .from(DateTime::from_timestamp(1788220800, 0).unwrap())
            .to(DateTime::from_timestamp(1788739200, 0).unwrap())
            .keywords(["error"])
            .build();
        let expected = Ok((
            vec![LogEntry::builder()
                .cursor("cursor1")
                .timestamp(DateTime::from_timestamp(1788784496, 0).unwrap())
                .severity(LogSeverity::Error)
                .severity_number(17)
                .body("internal server error")
                .service_name("shoppingcart")
                .build()],
            LogPagination::builder()
                .has_next_page(true)
                .has_previous_page(false)
                .start_cursor("cursor1")
                .end_cursor("cursor1")
                .build(),
        ));
        assert_eq!(test_client!(server).list_logs(&params).await, expected);
        assert_eq!(test_client!(server).list_logs(params).await, expected);
    }

    #[tokio::test]
    async fn list_saved_log_searches() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/saved-log-searches",
            response = json!({
                "results": [{
                    "id": "1234",
                    "name": "Errors",
                    "description": null,
                    "definition": {
                        "serviceName": "shoppingcart",
                        "serviceNamespace": null,
                        "keywords": ["error"],
                        "severities": ["ERROR"],
                        "traceId": null,
                        "bodyAttributes": [],
                    },
                    "createdAt": 1718802000,
                    "updatedAt": 1718888400,
                }],
            }),
        };
        assert_eq!(
            test_client!(server).list_saved_log_searches().await,
            Ok(vec![SavedLogSearch::builder()
                .id("1234")
                .value(
                    SavedLogSearchValue::builder()
                        .name("Errors")
                        .definition(
                            SavedLogSearchDefinition::builder()
                                .service_name("shoppingcart")
                                .keywords(vec!["error".to_owned()])
                                .severities(vec![LogSeverity::Error])
                                .build(),
                        )
                        .created_at(DateTime::from_timestamp(1718802000, 0).unwrap())
                        .updated_at(DateTime::from_timestamp(1718888400, 0).unwrap())
                        .build(),
                )
                .build()]),
        );
    }
}
