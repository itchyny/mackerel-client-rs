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
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MonitorValue {
    #[serde(rename_all = "camelCase")]
    Host {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        duration: u64,
        metric: String,
        operator: MonitorOperator,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        warning: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        critical: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_check_attempts: Option<u64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        exclude_scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
    #[serde(rename_all = "camelCase")]
    Connectivity {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        #[serde(
            default = "AlertStatus::critical",
            skip_serializing_if = "AlertStatus::is_critical"
        )]
        alert_status_on_gone: AlertStatus,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        exclude_scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
    #[serde(rename_all = "camelCase")]
    Service {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        service: ServiceName,
        duration: u64,
        metric: String,
        operator: MonitorOperator,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        warning: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        critical: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_check_attempts: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        missing_duration_warning: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        missing_duration_critical: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
    #[serde(rename_all = "camelCase")]
    External {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        url: String,
        #[serde(default)]
        method: ExternalMethod,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<ExternalHeader>>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        request_body: String,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        follow_redirect: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        service: Option<ServiceName>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        response_time_duration: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        response_time_warning: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        response_time_critical: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        contains_string: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_check_attempts: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        certification_expiration_warning: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        certification_expiration_critical: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        skip_certificate_verification: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
    #[serde(rename_all = "camelCase")]
    Expression {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        expression: String,
        operator: MonitorOperator,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        warning: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        critical: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
    #[serde(rename_all = "camelCase")]
    AnomalyDetection {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        warning_sensitivity: Option<AnomalyDetectionSensitivity>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        critical_sensitivity: Option<AnomalyDetectionSensitivity>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_check_attempts: Option<u64>,
        #[serde(default, with = "chrono::serde::ts_seconds_option")]
        training_period_from: Option<DateTime<Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
    #[serde(rename_all = "camelCase")]
    Query {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        query: String,
        legend: String,
        operator: MonitorOperator,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        warning: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        critical: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_mute: bool,
    },
}

impl MonitorValue {
    /// Returns the `name` of the monitor.
    pub fn name(&self) -> String {
        match *self {
            Self::Host { ref name, .. } => name.clone(),
            Self::Connectivity { ref name, .. } => name.clone(),
            Self::Service { ref name, .. } => name.clone(),
            Self::External { ref name, .. } => name.clone(),
            Self::Expression { ref name, .. } => name.clone(),
            Self::AnomalyDetection { ref name, .. } => name.clone(),
            Self::Query { ref name, .. } => name.clone(),
        }
    }

    /// Returns the `memo` of the monitor.
    pub fn memo(&self) -> String {
        match *self {
            Self::Host { ref memo, .. } => memo.clone(),
            Self::Connectivity { ref memo, .. } => memo.clone(),
            Self::Service { ref memo, .. } => memo.clone(),
            Self::External { ref memo, .. } => memo.clone(),
            Self::Expression { ref memo, .. } => memo.clone(),
            Self::AnomalyDetection { ref memo, .. } => memo.clone(),
            Self::Query { ref memo, .. } => memo.clone(),
        }
    }

    /// Returns the `is_mute` of the monitor.
    pub fn is_mute(&self) -> bool {
        match *self {
            Self::Host { is_mute, .. } => is_mute,
            Self::Connectivity { is_mute, .. } => is_mute,
            Self::Service { is_mute, .. } => is_mute,
            Self::External { is_mute, .. } => is_mute,
            Self::Expression { is_mute, .. } => is_mute,
            Self::AnomalyDetection { is_mute, .. } => is_mute,
            Self::Query { is_mute, .. } => is_mute,
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
    name: String,
    value: String,
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
            .value(MonitorValue::Host {
                name: "Example host monitor".to_string(),
                memo: "Monitor memo".to_string(),
                duration: 5,
                metric: "custom.foo.bar".to_string(),
                operator: MonitorOperator::GreaterThan,
                warning: Some(10.0),
                critical: Some(20.0),
                max_check_attempts: Some(5),
                scopes: vec!["service0".into()],
                exclude_scopes: vec!["service0:role3".into()],
                notification_interval: Some(30),
                is_mute: false,
            })
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
            .value(MonitorValue::Connectivity {
                name: "Example connectivity monitor".to_string(),
                memo: "Monitor memo".to_string(),
                alert_status_on_gone: AlertStatus::Warning,
                scopes: vec![],
                exclude_scopes: vec![],
                notification_interval: None,
                is_mute: false,
            })
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
            .value(MonitorValue::Service {
                name: "Example service monitor".to_string(),
                memo: "Monitor memo".to_string(),
                service: "service1".into(),
                duration: 5,
                metric: "custom.service.count".to_string(),
                operator: MonitorOperator::GreaterThan,
                warning: Some(100.0),
                critical: Some(200.0),
                max_check_attempts: Some(10),
                missing_duration_warning: Some(60),
                missing_duration_critical: Some(120),
                notification_interval: Some(30),
                is_mute: false,
            })
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
            .value(MonitorValue::External {
                name: "Example external monitor".to_string(),
                memo: "Monitor memo".to_string(),
                url: "https://example.com".to_string(),
                method: ExternalMethod::Get,
                headers: Some(vec![ExternalHeader::builder()
                    .name("Cache-Control")
                    .value("no-cache")
                    .build()]),
                request_body: "Request Body".to_owned(),
                follow_redirect: true,
                service: Some("service1".into()),
                response_time_duration: Some(5),
                response_time_warning: Some(3000.0),
                response_time_critical: Some(5000.0),
                contains_string: Some("Example Domain".to_string()),
                max_check_attempts: Some(5),
                certification_expiration_warning: Some(1200),
                certification_expiration_critical: Some(60),
                skip_certificate_verification: true,
                notification_interval: Some(60),
                is_mute: true,
            })
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
            "service": "service1",
            "responseTimeDuration": 5,
            "responseTimeWarning": 3000.0,
            "responseTimeCritical": 5000.0,
            "containsString": "Example Domain",
            "maxCheckAttempts": 5,
            "certificationExpirationWarning": 1200,
            "certificationExpirationCritical": 60,
            "skipCertificateVerification": true,
            "notificationInterval": 60,
            "isMute": true,
        })
    }

    fn expression_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor5")
            .value(MonitorValue::Expression {
                name: "Example expression monitor".to_string(),
                memo: "Monitor memo".to_string(),
                expression: "min(role(\"foo:bar\", \"custom.foo.bar\"))".to_string(),
                operator: MonitorOperator::LessThan,
                warning: Some(10.0),
                critical: None,
                notification_interval: None,
                is_mute: true,
            })
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
            "isMute": true
        })
    }

    fn anomaly_detection_monitor_example() -> Monitor {
        Monitor::builder()
            .id("monitor6")
            .value(MonitorValue::AnomalyDetection {
                name: "Example anomaly detection monitor".to_string(),
                memo: "Monitor memo".to_string(),
                scopes: vec!["service0:role0".into()],
                warning_sensitivity: Some(AnomalyDetectionSensitivity::Normal),
                critical_sensitivity: Some(AnomalyDetectionSensitivity::Insensitive),
                max_check_attempts: Some(3),
                training_period_from: Some(DateTime::from_timestamp(1580000000, 0).unwrap()),
                notification_interval: None,
                is_mute: true,
            })
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
            .value(MonitorValue::Query {
                name: "Example query monitor".to_string(),
                memo: "Monitor memo".to_string(),
                query: "container.cpu.utilization{label=\"value\"}".to_string(),
                legend: "cpu.utilization {{k8s.node.name}}".to_string(),
                operator: MonitorOperator::GreaterThan,
                warning: Some(75.0),
                critical: Some(90.0),
                notification_interval: None,
                is_mute: false,
            })
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
            format_url!("/api/v0/monitors/{}", monitor_id),
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
            format_url!("/api/v0/monitors/{}", monitor_id),
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
            format_url!("/api/v0/monitors/{}", monitor_id),
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
        MonitorValue::Connectivity {
            name: "Example connectivity monitor".to_string(),
            memo: "Monitor memo".to_string(),
            alert_status_on_gone: AlertStatus::Critical,
            scopes: vec![],
            exclude_scopes: vec![],
            notification_interval: None,
            is_mute: false,
        }
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

    #[async_std::test]
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

    #[async_std::test]
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

    #[async_std::test]
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

    #[async_std::test]
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

    #[async_std::test]
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
