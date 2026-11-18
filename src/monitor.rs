use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::alert::AlertStatus;
use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::role::RoleFullname;
use crate::service::ServiceName;

/// A monitor entity
pub type Monitor = Entity<MonitorValue>;

/// A monitor id
pub type MonitorId = Id<MonitorValue>;

/// A monitor value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MonitorValue {
    Host(HostMonitor),
    Connectivity(ConnectivityMonitor),
    Service(ServiceMonitor),
    External(ExternalMonitor),
    Expression(ExpressionMonitor),
    AnomalyDetection(AnomalyDetectionMonitor),
    Query(QueryMonitor),
}

/// A host metric monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct HostMonitor {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[builder(setter(!into))]
    pub duration: u64,
    pub metric: String,
    pub operator: MonitorOperator,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_check_attempts: Option<u64>,
    #[builder(
        default,
        setter(transform = |scopes: impl IntoIterator<Item = impl Into<MonitorScope>>| scopes
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<MonitorScope>,
    #[builder(
        default,
        setter(transform = |scopes: impl IntoIterator<Item = impl Into<MonitorScope>>| scopes
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_scopes: Vec<MonitorScope>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

/// A host connectivity monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConnectivityMonitor {
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[builder(default = AlertStatus::Critical)]
    #[serde(
        default = "AlertStatus::critical",
        skip_serializing_if = "AlertStatus::is_critical"
    )]
    pub alert_status_on_gone: AlertStatus,
    #[builder(
        default,
        setter(transform = |scopes: impl IntoIterator<Item = impl Into<MonitorScope>>| scopes
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<MonitorScope>,
    #[builder(
        default,
        setter(transform = |scopes: impl IntoIterator<Item = impl Into<MonitorScope>>| scopes
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_scopes: Vec<MonitorScope>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

/// A service metric monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ServiceMonitor {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    pub service: ServiceName,
    #[builder(setter(!into))]
    pub duration: u64,
    pub metric: String,
    pub operator: MonitorOperator,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_check_attempts: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_duration_warning: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_duration_critical: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

/// An external http monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ExternalMonitor {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    pub url: String,
    #[builder(default)]
    #[serde(default)]
    pub method: ExternalMethod,
    #[builder(
        default,
        setter(transform = |headers: impl IntoIterator<Item = ExternalHeader>| Some(headers
            .into_iter().collect::<Vec<_>>())),
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<ExternalHeader>>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request_body: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub follow_redirect: bool,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_status_code: Option<u16>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceName>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_time_duration: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_time_warning: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_time_critical: Option<f64>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains_string: Option<String>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_check_attempts: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification_expiration_warning: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification_expiration_critical: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub skip_certificate_verification: bool,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dualstack: Option<ExternalDualstack>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

/// An expression monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ExpressionMonitor {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    pub expression: String,
    pub operator: MonitorOperator,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluate_backward_minutes: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

/// An anomaly detection monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AnomalyDetectionMonitor {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[builder(
        default,
        setter(transform = |scopes: impl IntoIterator<Item = impl Into<MonitorScope>>| scopes
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<MonitorScope>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_sensitivity: Option<AnomalyDetectionSensitivity>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical_sensitivity: Option<AnomalyDetectionSensitivity>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_check_attempts: Option<u64>,
    #[builder(default, setter(strip_option))]
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub training_period_from: Option<DateTime<Utc>>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

/// A query monitor
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct QueryMonitor {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    pub query: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub legend: String,
    pub operator: MonitorOperator,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<f64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evaluate_backward_minutes: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_mute: bool,
}

impl From<HostMonitor> for MonitorValue {
    fn from(monitor: HostMonitor) -> Self {
        Self::Host(monitor)
    }
}

impl From<ConnectivityMonitor> for MonitorValue {
    fn from(monitor: ConnectivityMonitor) -> Self {
        Self::Connectivity(monitor)
    }
}

impl From<ServiceMonitor> for MonitorValue {
    fn from(monitor: ServiceMonitor) -> Self {
        Self::Service(monitor)
    }
}

impl From<ExternalMonitor> for MonitorValue {
    fn from(monitor: ExternalMonitor) -> Self {
        Self::External(monitor)
    }
}

impl From<ExpressionMonitor> for MonitorValue {
    fn from(monitor: ExpressionMonitor) -> Self {
        Self::Expression(monitor)
    }
}

impl From<AnomalyDetectionMonitor> for MonitorValue {
    fn from(monitor: AnomalyDetectionMonitor) -> Self {
        Self::AnomalyDetection(monitor)
    }
}

impl From<QueryMonitor> for MonitorValue {
    fn from(monitor: QueryMonitor) -> Self {
        Self::Query(monitor)
    }
}

impl MonitorValue {
    /// Returns the `name` of the monitor.
    pub fn name(&self) -> String {
        match self {
            Self::Host(monitor) => monitor.name.clone(),
            Self::Connectivity(monitor) => monitor.name.clone(),
            Self::Service(monitor) => monitor.name.clone(),
            Self::External(monitor) => monitor.name.clone(),
            Self::Expression(monitor) => monitor.name.clone(),
            Self::AnomalyDetection(monitor) => monitor.name.clone(),
            Self::Query(monitor) => monitor.name.clone(),
        }
    }

    /// Returns the `memo` of the monitor.
    pub fn memo(&self) -> String {
        match self {
            Self::Host(monitor) => monitor.memo.clone(),
            Self::Connectivity(monitor) => monitor.memo.clone(),
            Self::Service(monitor) => monitor.memo.clone(),
            Self::External(monitor) => monitor.memo.clone(),
            Self::Expression(monitor) => monitor.memo.clone(),
            Self::AnomalyDetection(monitor) => monitor.memo.clone(),
            Self::Query(monitor) => monitor.memo.clone(),
        }
    }

    /// Returns the `is_mute` of the monitor.
    pub fn is_mute(&self) -> bool {
        match self {
            Self::Host(monitor) => monitor.is_mute,
            Self::Connectivity(monitor) => monitor.is_mute,
            Self::Service(monitor) => monitor.is_mute,
            Self::External(monitor) => monitor.is_mute,
            Self::Expression(monitor) => monitor.is_mute,
            Self::AnomalyDetection(monitor) => monitor.is_mute,
            Self::Query(monitor) => monitor.is_mute,
        }
    }
}

/// Monitor type
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "camelCase")]
pub enum MonitorType {
    Connectivity,
    Host,
    Service,
    External,
    Check,
    Expression,
    AnomalyDetection,
    Query,
}

/// Monitor operator
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
pub enum MonitorOperator {
    #[strum(serialize = ">")]
    GreaterThan,
    #[strum(serialize = "<")]
    LessThan,
}

/// Monitor scope
#[derive(PartialEq, Eq, Copy, Clone, Debug, SerializeDisplay, DeserializeFromStr)]
pub enum MonitorScope {
    Service(ServiceName),
    Role(RoleFullname),
}

#[derive(PartialEq, Eq, Debug)]
pub struct ParseMonitorScopeError(String);

impl std::fmt::Display for ParseMonitorScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse monitor scope: {}", self.0)
    }
}

impl std::str::FromStr for MonitorScope {
    type Err = ParseMonitorScopeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.parse()
            .map(MonitorScope::Role)
            .or(s.parse().map(MonitorScope::Service))
            .map_err(|_| ParseMonitorScopeError(s.to_owned()))
    }
}

impl From<&str> for MonitorScope {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl From<String> for MonitorScope {
    fn from(s: String) -> Self {
        s.parse().unwrap()
    }
}

impl std::fmt::Display for MonitorScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Service(ref service_name) => service_name.fmt(f),
            Self::Role(ref role_fullname) => role_fullname.fmt(f),
        }
    }
}

impl From<MonitorScope> for String {
    fn from(val: MonitorScope) -> Self {
        val.to_string()
    }
}

/// HTTP method for external http monitor
#[derive(
    PartialEq,
    Eq,
    Copy,
    Clone,
    Default,
    Debug,
    Display,
    EnumString,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ExternalMethod {
    #[default]
    Get,
    Post,
    Put,
    Delete,
}

/// HTTP header for external http monitor
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct ExternalHeader {
    pub name: String,
    pub value: String,
}

/// IP version for external http monitor
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalDualstack {
    Ipv4,
    Ipv6,
    Auto,
}

/// Anomaly detection sensitivity
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum AnomalyDetectionSensitivity {
    Insensitive,
    Normal,
    Sensitive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn host_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor1")
            .value(
                HostMonitor::builder()
                    .name("Example host monitor")
                    .memo("Monitor memo")
                    .duration(5)
                    .metric("custom.foo.bar")
                    .operator(MonitorOperator::GreaterThan)
                    .warning(10.0)
                    .critical(20.0)
                    .max_check_attempts(5)
                    .scopes(["service0"])
                    .exclude_scopes(["service0:role3"])
                    .notification_interval(30)
                    .build(),
            )
            .build()
    }

    fn host_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "host",
            "id": "monitor1",
            "name": "Example host monitor",
            "memo": "Monitor memo",
            "duration": 5,
            "metric": "custom.foo.bar",
            "operator": ">",
            "warning": 10.0,
            "critical": 20.0,
            "maxCheckAttempts": 5,
            "scopes": ["service0"],
            "excludeScopes": ["service0:role3"],
            "notificationInterval": 30,
        })
    }

    fn connectivity_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor2")
            .value(
                ConnectivityMonitor::builder()
                    .name("Example connectivity monitor")
                    .memo("Monitor memo")
                    .alert_status_on_gone(AlertStatus::Warning)
                    .build(),
            )
            .build()
    }

    fn connectivity_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "connectivity",
            "id": "monitor2",
            "name": "Example connectivity monitor",
            "memo": "Monitor memo",
            "alertStatusOnGone": "WARNING",
        })
    }

    fn service_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor3")
            .value(
                ServiceMonitor::builder()
                    .name("Example service monitor")
                    .memo("Monitor memo")
                    .service("service1")
                    .duration(5)
                    .metric("custom.service.count")
                    .operator(MonitorOperator::GreaterThan)
                    .warning(100.0)
                    .critical(200.0)
                    .max_check_attempts(10)
                    .missing_duration_warning(60)
                    .missing_duration_critical(120)
                    .notification_interval(30)
                    .build(),
            )
            .build()
    }

    fn service_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "service",
            "id": "monitor3",
            "name": "Example service monitor",
            "memo": "Monitor memo",
            "service": "service1",
            "duration": 5,
            "metric": "custom.service.count",
            "operator": ">",
            "warning": 100.0,
            "critical": 200.0,
            "maxCheckAttempts": 10,
            "missingDurationWarning": 60,
            "missingDurationCritical": 120,
            "notificationInterval": 30,
        })
    }

    fn external_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor4")
            .value(
                ExternalMonitor::builder()
                    .name("Example external monitor")
                    .memo("Monitor memo")
                    .url("https://example.com")
                    .method(ExternalMethod::Get)
                    .headers([ExternalHeader::builder()
                        .name("Cache-Control")
                        .value("no-cache")
                        .build()])
                    .request_body("Request Body")
                    .follow_redirect(true)
                    .expected_status_code(200)
                    .service("service1")
                    .response_time_duration(5)
                    .response_time_warning(3000.0)
                    .response_time_critical(5000.0)
                    .contains_string("Example Domain")
                    .max_check_attempts(5)
                    .certification_expiration_warning(1200)
                    .certification_expiration_critical(60)
                    .skip_certificate_verification(true)
                    .dualstack(ExternalDualstack::Auto)
                    .notification_interval(60)
                    .is_mute(true)
                    .build(),
            )
            .build()
    }

    fn external_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "external",
            "id": "monitor4",
            "name": "Example external monitor",
            "memo": "Monitor memo",
            "url": "https://example.com",
            "method": "GET",
            "headers": [{ "name": "Cache-Control", "value": "no-cache" }],
            "requestBody": "Request Body",
            "followRedirect": true,
            "expectedStatusCode": 200,
            "service": "service1",
            "responseTimeDuration": 5,
            "responseTimeWarning": 3000.0,
            "responseTimeCritical": 5000.0,
            "containsString": "Example Domain",
            "maxCheckAttempts": 5,
            "certificationExpirationWarning": 1200,
            "certificationExpirationCritical": 60,
            "skipCertificateVerification": true,
            "dualstack": "auto",
            "notificationInterval": 60,
            "isMute": true,
        })
    }

    fn expression_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor5")
            .value(
                ExpressionMonitor::builder()
                    .name("Example expression monitor")
                    .memo("Monitor memo")
                    .expression("min(role(\"foo:bar\", \"custom.foo.bar\"))")
                    .operator(MonitorOperator::LessThan)
                    .warning(10.0)
                    .evaluate_backward_minutes(5)
                    .is_mute(true)
                    .build(),
            )
            .build()
    }

    fn expression_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "expression",
            "id": "monitor5",
            "name": "Example expression monitor",
            "memo": "Monitor memo",
            "expression": "min(role(\"foo:bar\", \"custom.foo.bar\"))",
            "operator": "<",
            "warning": 10.0,
            "evaluateBackwardMinutes": 5,
            "isMute": true
        })
    }

    fn anomaly_detection_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor6")
            .value(
                AnomalyDetectionMonitor::builder()
                    .name("Example anomaly detection monitor")
                    .memo("Monitor memo")
                    .scopes(["service0:role0"])
                    .warning_sensitivity(AnomalyDetectionSensitivity::Normal)
                    .critical_sensitivity(AnomalyDetectionSensitivity::Insensitive)
                    .max_check_attempts(3)
                    .training_period_from(DateTime::from_timestamp(1580000000, 0).unwrap())
                    .is_mute(true)
                    .build(),
            )
            .build()
    }

    fn anomaly_detection_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "anomalyDetection",
            "id": "monitor6",
            "name": "Example anomaly detection monitor",
            "memo": "Monitor memo",
            "scopes": ["service0:role0"],
            "warningSensitivity": "normal",
            "criticalSensitivity": "insensitive",
            "maxCheckAttempts": 3,
            "trainingPeriodFrom": 1580000000,
            "isMute": true
        })
    }

    fn query_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor7")
            .value(
                QueryMonitor::builder()
                    .name("Example query monitor")
                    .memo("Monitor memo")
                    .query("container.cpu.utilization{label=\"value\"}")
                    .legend("cpu.utilization {{k8s.node.name}}")
                    .operator(MonitorOperator::GreaterThan)
                    .warning(75.0)
                    .critical(90.0)
                    .build(),
            )
            .build()
    }

    fn query_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "query",
            "id": "monitor7",
            "name": "Example query monitor",
            "memo": "Monitor memo",
            "query": "container.cpu.utilization{label=\"value\"}",
            "legend": "cpu.utilization {{k8s.node.name}}",
            "operator": ">",
            "warning": 75.0,
            "critical": 90.0,
        })
    }

    #[rstest]
    #[case(host_monitor_example(), host_monitor_json_example())]
    #[case(connectivity_monitor_example(), connectivity_monitor_json_example())]
    #[case(service_monitor_example(), service_monitor_json_example())]
    #[case(external_monitor_example(), external_monitor_json_example())]
    #[case(expression_monitor_example(), expression_monitor_json_example())]
    #[case(
        anomaly_detection_monitor_example(),
        anomaly_detection_monitor_json_example()
    )]
    #[case(query_monitor_example(), query_monitor_json_example())]
    #[case(
        Monitor::builder()
            .id("monitor8")
            .value(ConnectivityMonitor::builder().build())
            .build(),
        json!({ "type": "connectivity", "id": "monitor8" }),
    )]
    #[case(
        Monitor::builder()
            .id("monitor9")
            .value(
                QueryMonitor::builder()
                    .name("Example query monitor without legend")
                    .query("container.cpu.utilization")
                    .operator(MonitorOperator::GreaterThan)
                    .build(),
            )
            .build(),
        json!({
            "type": "query",
            "id": "monitor9",
            "name": "Example query monitor without legend",
            "query": "container.cpu.utilization",
            "operator": ">",
        }),
    )]
    fn test_monitor_json(#[case] monitor: Monitor, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&monitor).unwrap(), json);
        assert_eq!(monitor, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(host_monitor_example(), "Example host monitor")]
    #[case(connectivity_monitor_example(), "Example connectivity monitor")]
    #[case(service_monitor_example(), "Example service monitor")]
    #[case(external_monitor_example(), "Example external monitor")]
    #[case(expression_monitor_example(), "Example expression monitor")]
    #[case(
        anomaly_detection_monitor_example(),
        "Example anomaly detection monitor"
    )]
    #[case(query_monitor_example(), "Example query monitor")]
    fn test_monitor_name(#[case] monitor: Monitor, #[case] name_str: &str) {
        assert_eq!(monitor.name(), name_str);
    }

    #[rstest]
    #[case(host_monitor_example())]
    #[case(connectivity_monitor_example())]
    #[case(service_monitor_example())]
    #[case(external_monitor_example())]
    #[case(expression_monitor_example())]
    #[case(anomaly_detection_monitor_example())]
    #[case(query_monitor_example())]
    fn test_monitor_memo(#[case] monitor: Monitor) {
        assert_eq!(monitor.memo(), "Monitor memo");
    }

    #[rstest]
    #[case(host_monitor_example(), false)]
    #[case(connectivity_monitor_example(), false)]
    #[case(service_monitor_example(), false)]
    #[case(external_monitor_example(), true)]
    #[case(expression_monitor_example(), true)]
    #[case(anomaly_detection_monitor_example(), true)]
    #[case(query_monitor_example(), false)]
    fn test_monitor_is_mute(#[case] monitor: Monitor, #[case] is_mute: bool) {
        assert_eq!(monitor.is_mute(), is_mute);
    }

    #[rstest]
    #[case(MonitorType::Connectivity, "connectivity")]
    #[case(MonitorType::Host, "host")]
    #[case(MonitorType::Service, "service")]
    #[case(MonitorType::External, "external")]
    #[case(MonitorType::Check, "check")]
    #[case(MonitorType::Expression, "expression")]
    #[case(MonitorType::AnomalyDetection, "anomalyDetection")]
    #[case(MonitorType::Query, "query")]
    fn test_monitor_type(#[case] monitor_type: MonitorType, #[case] monitor_type_str: &str) {
        assert_eq!(monitor_type.to_string(), monitor_type_str);
        assert_eq!(monitor_type, monitor_type_str.parse().unwrap());
        assert_eq!(
            monitor_type,
            serde_json::from_value(monitor_type_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(monitor_type).unwrap(),
            monitor_type_str
        );
    }

    #[rstest]
    #[case(MonitorOperator::GreaterThan, ">")]
    #[case(MonitorOperator::LessThan, "<")]
    fn test_monitor_operator(
        #[case] monitor_operator: MonitorOperator,
        #[case] monitor_operator_str: &str,
    ) {
        assert_eq!(monitor_operator.to_string(), monitor_operator_str);
        assert_eq!(monitor_operator, monitor_operator_str.parse().unwrap());
        assert_eq!(
            monitor_operator,
            serde_json::from_value(monitor_operator_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(monitor_operator).unwrap(),
            monitor_operator_str
        );
    }

    #[rstest]
    #[case("ExampleService".into(), "ExampleService")]
    #[case("ExampleService:ExampleRole".into(), "ExampleService:ExampleRole")]
    fn test_monitor_scope(#[case] monitor_scope: MonitorScope, #[case] monitor_scope_str: &str) {
        assert_eq!(monitor_scope.to_string(), monitor_scope_str);
        assert_eq!(monitor_scope, monitor_scope_str.parse().unwrap());
        assert_eq!(
            monitor_scope,
            serde_json::from_value(monitor_scope_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(monitor_scope).unwrap(),
            monitor_scope_str
        );
    }

    #[rstest]
    #[case("")]
    #[case(":")]
    #[case("ExampleService:")]
    #[case(":ExampleRole")]
    fn test_monitor_scope_error(#[case] monitor_scope_str: &str) {
        assert_eq!(
            Err(ParseMonitorScopeError(monitor_scope_str.to_string())),
            monitor_scope_str.parse::<MonitorScope>()
        );
    }

    #[rstest]
    #[case(ExternalMethod::Get, "GET")]
    #[case(ExternalMethod::Post, "POST")]
    #[case(ExternalMethod::Put, "PUT")]
    #[case(ExternalMethod::Delete, "DELETE")]
    #[case(ExternalMethod::default(), "GET")]
    fn test_external_method(
        #[case] external_method: ExternalMethod,
        #[case] external_method_str: &str,
    ) {
        assert_eq!(external_method.to_string(), external_method_str);
        assert_eq!(external_method, external_method_str.parse().unwrap());
        assert_eq!(
            external_method,
            serde_json::from_value(external_method_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(external_method).unwrap(),
            external_method_str
        );
    }

    #[rstest]
    #[case(AnomalyDetectionSensitivity::Insensitive, "insensitive")]
    #[case(AnomalyDetectionSensitivity::Normal, "normal")]
    #[case(AnomalyDetectionSensitivity::Sensitive, "sensitive")]
    fn test_anomaly_detection_sensitivity(
        #[case] anomaly_detection_sensitivity: AnomalyDetectionSensitivity,
        #[case] anomaly_detection_sensitivity_str: &str,
    ) {
        assert_eq!(
            anomaly_detection_sensitivity.to_string(),
            anomaly_detection_sensitivity_str
        );
        assert_eq!(
            anomaly_detection_sensitivity,
            anomaly_detection_sensitivity_str.parse().unwrap()
        );
        assert_eq!(
            anomaly_detection_sensitivity,
            serde_json::from_value(anomaly_detection_sensitivity_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(anomaly_detection_sensitivity).unwrap(),
            anomaly_detection_sensitivity_str
        );
    }
}

impl Client {
    /// Fetches all the monitors.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#get>.
    pub async fn list_monitors(&self) -> Result<Vec<Monitor>> {
        self.request(
            Method::GET,
            "/api/v0/monitors",
            query_params![],
            request_body![],
            response_body! { monitors: Vec<Monitor> },
        )
        .await
    }

    /// Creates a new monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#create>.
    pub async fn create_monitor(
        &self,
        monitor_value: impl Borrow<MonitorValue>,
    ) -> Result<Monitor> {
        self.request(
            Method::POST,
            "/api/v0/monitors",
            query_params![],
            request_body!(monitor_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Gets a monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#get>.
    pub async fn get_monitor(&self, monitor_id: impl Into<MonitorId>) -> Result<Monitor> {
        self.request(
            Method::GET,
            format!("/api/v0/monitors/{}", monitor_id.into()),
            query_params![],
            request_body![],
            response_body! { monitor: Monitor },
        )
        .await
    }

    /// Updates a monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#update>.
    pub async fn update_monitor(
        &self,
        monitor_id: impl Into<MonitorId>,
        monitor_value: impl Borrow<MonitorValue>,
    ) -> Result<Monitor> {
        self.request(
            Method::PUT,
            format!("/api/v0/monitors/{}", monitor_id.into()),
            query_params![],
            request_body!(monitor_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Deletes a monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#delete>.
    pub async fn delete_monitor(&self, monitor_id: impl Into<MonitorId>) -> Result<Monitor> {
        self.request(
            Method::DELETE,
            format!("/api/v0/monitors/{}", monitor_id.into()),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::monitor::*;
    use crate::tests::*;

    fn value_example() -> MonitorValue {
        ConnectivityMonitor::builder()
            .name("Example connectivity monitor")
            .memo("Monitor memo")
            .build()
            .into()
    }

    fn entity_example() -> Monitor {
        Monitor {
            id: MonitorId::from("monitor0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "type": "connectivity",
            "name": "Example connectivity monitor",
            "memo": "Monitor memo",
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("monitor0");
        json
    }

    #[tokio::test]
    async fn list_monitors() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/monitors",
            response = json!({
                "monitors": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_monitors().await,
            Ok(vec![entity_example()]),
        );
    }

    #[tokio::test]
    async fn create_monitor() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/monitors",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).create_monitor(value_example()).await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server).create_monitor(&value_example()).await,
            Ok(entity_example()),
        );
    }

    #[tokio::test]
    async fn get_monitor() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/monitors/monitor0",
            response = json!({ "monitor": entity_json_example() }),
        };
        assert_eq!(
            test_client!(server).get_monitor("monitor0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .get_monitor(MonitorId::from("monitor0"))
                .await,
            Ok(entity_example()),
        );
    }

    #[tokio::test]
    async fn update_monitor() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/monitors/monitor0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_monitor("monitor0", value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_monitor(MonitorId::from("monitor0"), &value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[tokio::test]
    async fn delete_monitor() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/monitors/monitor0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).delete_monitor("monitor0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_monitor(MonitorId::from("monitor0"))
                .await,
            Ok(entity_example()),
        );
    }
}
