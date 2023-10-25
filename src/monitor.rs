use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::role::RoleFullname;
use crate::service::ServiceName;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

/// A monitor
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
        warning: Option<f64>,
        critical: Option<f64>,
        is_mute: Option<bool>,
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        exclude_scopes: Vec<MonitorScope>,
    },
    #[serde(rename_all = "camelCase")]
    Connectivity {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        is_mute: Option<bool>,
        notification_interval: Option<u64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        scopes: Vec<MonitorScope>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        exclude_scopes: Vec<MonitorScope>,
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
        warning: Option<f64>,
        critical: Option<f64>,
        is_mute: Option<bool>,
        notification_interval: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    External {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        method: Option<ExternalMethod>,
        url: String,
        request_body: Option<String>,
        headers: Option<Vec<ExternalHeader>>,
        service: Option<ServiceName>,
        response_time_duration: Option<u64>,
        response_time_warning: Option<f64>,
        response_time_critical: Option<f64>,
        contains_string: Option<String>,
        max_check_attempts: Option<u64>,
        certification_expiration_warning: Option<u64>,
        certification_expiration_critical: Option<u64>,
        skip_certificate_verification: Option<bool>,
        is_mute: Option<bool>,
        notification_interval: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    Expression {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        expression: String,
        operator: MonitorOperator,
        warning: Option<f64>,
        critical: Option<f64>,
        is_mute: Option<bool>,
        notification_interval: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    AnomalyDetection {
        name: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        memo: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        scopes: Vec<MonitorScope>,
        warning_sensitivity: Option<AnomalyDetectionSensitivity>,
        critical_sensitivity: Option<AnomalyDetectionSensitivity>,
        max_check_attempts: Option<u64>,
        #[serde(default, with = "chrono::serde::ts_seconds_option")]
        training_period_from: Option<DateTime<Utc>>,
        is_mute: Option<bool>,
        notification_interval: Option<u64>,
    },
}

impl MonitorValue {
    /// Returns the name of the monitor.
    pub fn name(&self) -> String {
        match *self {
            MonitorValue::Host { ref name, .. } => name.clone(),
            MonitorValue::Connectivity { ref name, .. } => name.clone(),
            MonitorValue::Service { ref name, .. } => name.clone(),
            MonitorValue::External { ref name, .. } => name.clone(),
            MonitorValue::Expression { ref name, .. } => name.clone(),
            MonitorValue::AnomalyDetection { ref name, .. } => name.clone(),
        }
    }
}

/// Monitor types
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
}

/// Monitor operators
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
            .map_err(|_| ParseMonitorScopeError(s.to_string()))
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

impl Into<String> for MonitorScope {
    fn into(self: Self) -> String {
        self.to_string()
    }
}

/// HTTP methods for external http monitors
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ExternalMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// HTTP headers for external http monitors
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
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
    use crate::monitor::*;
    use rstest::rstest;
    use serde_json::json;

    fn host_monitor_example() -> Monitor {
        Monitor {
            id: "abcde1".into(),
            value: MonitorValue::Host {
                name: "Monitor custom.foo.bar".to_string(),
                memo: "Monitor memo".to_string(),
                duration: 5,
                metric: "custom.foo.bar".to_string(),
                operator: MonitorOperator::GreaterThan,
                warning: Some(10.0),
                critical: Some(20.0),
                is_mute: Some(false),
                notification_interval: Some(30),
                scopes: vec!["service0".into()],
                exclude_scopes: vec!["service0:role3".into()],
            },
        }
    }

    fn host_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "host",
            "id": "abcde1",
            "name": "Monitor custom.foo.bar",
            "memo": "Monitor memo",
            "duration": 5,
            "metric": "custom.foo.bar",
            "operator": ">",
            "warning": 10.0,
            "critical": 20.0,
            "isMute": false,
            "notificationInterval": 30,
            "scopes": ["service0"],
            "excludeScopes": ["service0:role3"]
        })
    }

    fn connectivity_monitor_example() -> Monitor {
        Monitor {
            id: "abcde2".into(),
            value: MonitorValue::Connectivity {
                name: "connectivity".to_string(),
                memo: "Monitor memo".to_string(),
                is_mute: Some(false),
                notification_interval: None,
                scopes: vec![],
                exclude_scopes: vec![],
            },
        }
    }

    fn connectivity_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "connectivity",
            "id": "abcde2",
            "name": "connectivity",
            "memo": "Monitor memo",
            "isMute": false
        })
    }

    fn service_monitor_example() -> Monitor {
        Monitor {
            id: "abcde3".into(),
            value: MonitorValue::Service {
                name: "Service count".to_string(),
                memo: "Monitor memo".to_string(),
                service: "service1".into(),
                duration: 5,
                metric: "custom.service.count".to_string(),
                operator: MonitorOperator::GreaterThan,
                warning: Some(100.0),
                critical: Some(200.0),
                is_mute: Some(false),
                notification_interval: Some(30),
            },
        }
    }

    fn service_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "service",
            "id": "abcde3",
            "name": "Service count",
            "memo": "Monitor memo",
            "service": "service1",
            "duration": 5,
            "metric": "custom.service.count",
            "operator": ">",
            "warning": 100.0,
            "critical": 200.0,
            "isMute": false,
            "notificationInterval": 30
        })
    }

    fn external_monitor_example() -> Monitor {
        Monitor {
            id: "abcde4".into(),
            value: MonitorValue::External {
                name: "Example external monitor".to_string(),
                memo: "Monitor memo".to_string(),
                method: Some(ExternalMethod::Get),
                url: "https://example.com".to_string(),
                request_body: Some("Request Body".to_string()),
                headers: Some(vec![ExternalHeader {
                    name: "Cache-Control".to_string(),
                    value: "no-cache".to_string(),
                }]),
                service: Some("service1".into()),
                response_time_duration: Some(5),
                response_time_warning: Some(3000.0),
                response_time_critical: Some(5000.0),
                contains_string: Some("Example Domain".to_string()),
                max_check_attempts: Some(5),
                certification_expiration_warning: Some(1200),
                certification_expiration_critical: Some(60),
                skip_certificate_verification: Some(true),
                is_mute: Some(true),
                notification_interval: Some(60),
            },
        }
    }

    fn external_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "external",
            "id": "abcde4",
            "name": "Example external monitor",
            "memo": "Monitor memo",
            "method": "GET",
            "url": "https://example.com",
            "requestBody": "Request Body",
            "headers": [{ "name": "Cache-Control", "value": "no-cache" }],
            "service": "service1",
            "responseTimeDuration": 5,
            "responseTimeWarning": 3000.0,
            "responseTimeCritical": 5000.0,
            "containsString": "Example Domain",
            "maxCheckAttempts": 5,
            "certificationExpirationWarning": 1200,
            "certificationExpirationCritical": 60,
            "skipCertificateVerification": true,
            "isMute": true,
            "notificationInterval": 60
        })
    }

    fn expression_monitor_example() -> Monitor {
        Monitor {
            id: "abcde5".into(),
            value: MonitorValue::Expression {
                name: "Example expression monitor".to_string(),
                memo: "Monitor memo".to_string(),
                expression: "min(role(\"foo:bar\", \"custom.foo.bar\"))".to_string(),
                operator: MonitorOperator::LessThan,
                warning: Some(10.0),
                critical: None,
                is_mute: Some(false),
                notification_interval: None,
            },
        }
    }

    fn expression_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "expression",
            "id": "abcde5",
            "name": "Example expression monitor",
            "memo": "Monitor memo",
            "expression": "min(role(\"foo:bar\", \"custom.foo.bar\"))",
            "operator": "<",
            "warning": 10.0,
            "isMute": false
        })
    }

    fn anomaly_detection_monitor_example() -> Monitor {
        Monitor {
            id: "abcde6".into(),
            value: MonitorValue::AnomalyDetection {
                name: "Example Anomaly Detection monitor".to_string(),
                memo: "".to_string(),
                scopes: vec!["service0:role0".into()],
                warning_sensitivity: Some(AnomalyDetectionSensitivity::Normal),
                critical_sensitivity: Some(AnomalyDetectionSensitivity::Insensitive),
                max_check_attempts: Some(3),
                training_period_from: Some(DateTime::from_timestamp(1580000000, 0).unwrap()),
                is_mute: Some(false),
                notification_interval: None,
            },
        }
    }

    fn anomaly_detection_monitor_json_example() -> serde_json::Value {
        json!({
            "type": "anomalyDetection",
            "id": "abcde6",
            "name": "Example Anomaly Detection monitor",
            "scopes": ["service0:role0"],
            "warningSensitivity": "normal",
            "criticalSensitivity": "insensitive",
            "maxCheckAttempts": 3,
            "trainingPeriodFrom": 1580000000,
            "isMute": false
        })
    }

    fn monitor_examples() -> Vec<(Monitor, serde_json::Value)> {
        vec![
            (host_monitor_example(), host_monitor_json_example()),
            (
                connectivity_monitor_example(),
                connectivity_monitor_json_example(),
            ),
            (service_monitor_example(), service_monitor_json_example()),
            (external_monitor_example(), external_monitor_json_example()),
            (
                expression_monitor_example(),
                expression_monitor_json_example(),
            ),
            (
                anomaly_detection_monitor_example(),
                anomaly_detection_monitor_json_example(),
            ),
        ]
    }

    #[test]
    fn monitor_name() {
        assert_eq!(
            host_monitor_example().name(),
            "Monitor custom.foo.bar".to_string()
        );
        assert_eq!(
            connectivity_monitor_example().name(),
            "connectivity".to_string()
        );
        assert_eq!(
            service_monitor_example().name(),
            "Service count".to_string()
        );
        assert_eq!(
            external_monitor_example().name(),
            "Example external monitor".to_string()
        );
        assert_eq!(
            expression_monitor_example().name(),
            "Example expression monitor".to_string()
        );
        assert_eq!(
            anomaly_detection_monitor_example().name(),
            "Example Anomaly Detection monitor".to_string()
        );
    }

    #[test]
    fn serialize_monitor() {
        for (monitor, json) in monitor_examples() {
            assert_eq!(json, serde_json::to_value(monitor).unwrap());
        }
    }

    #[test]
    fn deserialize_monitor() {
        for (monitor, json) in monitor_examples() {
            assert_eq!(monitor, serde_json::from_value(json).unwrap());
        }
    }

    #[rstest]
    #[case(MonitorType::Connectivity, "connectivity")]
    #[case(MonitorType::Host, "host")]
    #[case(MonitorType::Service, "service")]
    #[case(MonitorType::External, "external")]
    #[case(MonitorType::Check, "check")]
    #[case(MonitorType::Expression, "expression")]
    #[case(MonitorType::AnomalyDetection, "anomalyDetection")]
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

#[derive(Deserialize)]
struct ListMonitorsResponse {
    monitors: Vec<Monitor>,
}

impl client::Client {
    /// Fetches all the monitors.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#get>.
    pub async fn list_monitors(&self) -> Result<Vec<Monitor>> {
        self.request(
            Method::GET,
            "/api/v0/monitors",
            vec![],
            client::empty_body(),
            |res: ListMonitorsResponse| res.monitors,
        )
        .await
    }

    /// Creates a new monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#create>.
    pub async fn create_monitor(&self, monitor_value: MonitorValue) -> Result<Monitor> {
        self.request(
            Method::POST,
            "/api/v0/monitors",
            vec![],
            Some(monitor_value),
            |monitor| monitor,
        )
        .await
    }

    /// Updates a monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#update>.
    pub async fn update_monitor(
        &self,
        monitor_id: MonitorId,
        monitor_value: MonitorValue,
    ) -> Result<Monitor> {
        self.request(
            Method::PUT,
            format!("/api/v0/monitors/{}", monitor_id),
            vec![],
            Some(monitor_value),
            |monitor| monitor,
        )
        .await
    }

    /// Deletes a monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/monitors#delete>.
    pub async fn delete_monitor(&self, monitor_id: MonitorId) -> Result<Monitor> {
        self.request(
            Method::DELETE,
            format!("/api/v0/monitors/{}", monitor_id),
            vec![],
            client::empty_body(),
            |monitor| monitor,
        )
        .await
    }
}
