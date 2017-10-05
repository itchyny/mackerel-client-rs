use std::fmt;
use reqwest::Method::*;
use client;
use errors::*;

/// A monitor
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Monitor {
    #[serde(rename_all = "camelCase")]
    Host {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        duration: u64,
        metric: String,
        operator: Operator,
        warning: f64,
        critical: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_mute: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        scopes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclude_scopes: Option<Vec<String>>,
    },
    #[serde(rename_all = "camelCase")]
    Connectivity {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_mute: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        scopes: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exclude_scopes: Option<Vec<String>>,
    },
    #[serde(rename_all = "camelCase")]
    Service {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        service: String,
        duration: u64,
        metric: String,
        operator: Operator,
        warning: f64,
        critical: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_mute: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    External {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        method: Option<ExternalMethod>,
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        request_body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<ExternalHeader>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        service: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time_duration: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time_warning: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        response_time_critical: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        contains_string: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_check_attempts: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        certification_expiration_warning: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        certification_expiration_critical: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        skip_certificate_verification: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_mute: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    Expression {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        memo: Option<String>,
        expression: String,
        operator: Operator,
        warning: f64,
        critical: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_mute: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notification_interval: Option<u64>,
    },
}

impl Monitor {
    /// Returns the id of the monitor.
    pub fn get_id(&self) -> Option<String> {
        match *self {
            Monitor::Host { ref id, .. } => id.clone(),
            Monitor::Connectivity { ref id, .. } => id.clone(),
            Monitor::Service { ref id, .. } => id.clone(),
            Monitor::External { ref id, .. } => id.clone(),
            Monitor::Expression { ref id, .. } => id.clone(),
        }
    }

    /// Returns the name of the monitor.
    pub fn get_name(&self) -> String {
        match *self {
            Monitor::Host { ref name, .. } => name.clone(),
            Monitor::Connectivity { ref name, .. } => name.clone(),
            Monitor::Service { ref name, .. } => name.clone(),
            Monitor::External { ref name, .. } => name.clone(),
            Monitor::Expression { ref name, .. } => name.clone(),
        }
    }
}

/// Monitor types
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitorType {
    Connectivity,
    Host,
    Service,
    External,
    Check,
    Expression,
}

impl fmt::Display for MonitorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MonitorType::Connectivity => write!(f, "connectivity"),
            MonitorType::Host => write!(f, "host"),
            MonitorType::Service => write!(f, "service"),
            MonitorType::External => write!(f, "external"),
            MonitorType::Check => write!(f, "check"),
            MonitorType::Expression => write!(f, "expression"),
        }
    }
}

/// Monitor operators
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::GreaterThan => write!(f, ">"),
            Operator::LessThan => write!(f, "<"),
        }
    }
}

/// HTTP methods for external http monitors
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl fmt::Display for ExternalMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExternalMethod::Get => write!(f, "GET"),
            ExternalMethod::Post => write!(f, "POST"),
            ExternalMethod::Put => write!(f, "PUT"),
            ExternalMethod::Delete => write!(f, "DELETE"),
        }
    }
}

/// HTTP headers for external http monitors
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ExternalHeader {
    name: String,
    value: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use monitor::*;

    fn host_monitor_example() -> Monitor {
        Monitor::Host {
            id: Some("abcde1".to_string()),
            name: "Monitor custom.foo.bar".to_string(),
            memo: Some("Monitor memo".to_string()),
            duration: 5,
            metric: "custom.foo.bar".to_string(),
            operator: Operator::GreaterThan,
            warning: 10.0,
            critical: 20.0,
            is_mute: Some(false),
            notification_interval: Some(30),
            scopes: Some(vec!["service0".to_string()]),
            exclude_scopes: Some(vec!["service0:role3".to_string()]),
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
        Monitor::Connectivity {
            id: Some("abcde2".to_string()),
            name: "connectivity".to_string(),
            memo: Some("Monitor memo".to_string()),
            is_mute: Some(false),
            notification_interval: None,
            scopes: None,
            exclude_scopes: None,
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
        Monitor::Service {
            id: Some("abcde3".to_string()),
            name: "Service count".to_string(),
            memo: Some("Monitor memo".to_string()),
            service: "service1".to_string(),
            duration: 5,
            metric: "custom.service.count".to_string(),
            operator: Operator::GreaterThan,
            warning: 100.0,
            critical: 200.0,
            is_mute: Some(false),
            notification_interval: Some(30),
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
        Monitor::External {
            id: Some("abcde4".to_string()),
            name: "Example external monitor".to_string(),
            memo: Some("Monitor memo".to_string()),
            method: Some(ExternalMethod::Get),
            url: "https://example.com".to_string(),
            request_body: Some("Request Body".to_string()),
            headers: Some(vec![
                ExternalHeader {
                    name: "Cache-Control".to_string(),
                    value: "no-cache".to_string(),
                },
            ]),
            service: Some("service1".to_string()),
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
        Monitor::Expression {
            id: Some("abcde5".to_string()),
            name: "Example expression monitor".to_string(),
            memo: Some("Monitor memo".to_string()),
            expression: "min(role(\"foo:bar\", \"custom.foo.bar\"))".to_string(),
            operator: Operator::LessThan,
            warning: 10.0,
            critical: 20.0,
            is_mute: Some(false),
            notification_interval: None,
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
            "critical": 20.0,
            "isMute": false
        })
    }

    fn monitor_examples() -> Vec<(Monitor, serde_json::Value)> {
        vec![
            (host_monitor_example(), host_monitor_json_example()),
            (connectivity_monitor_example(), connectivity_monitor_json_example()),
            (service_monitor_example(), service_monitor_json_example()),
            (external_monitor_example(), external_monitor_json_example()),
            (expression_monitor_example(), expression_monitor_json_example()),
        ]
    }

    #[test]
    fn monitor_id() {
        assert_eq!(host_monitor_example().get_id(), Some("abcde1".to_string()));
        assert_eq!(connectivity_monitor_example().get_id(), Some("abcde2".to_string()));
        assert_eq!(service_monitor_example().get_id(), Some("abcde3".to_string()));
        assert_eq!(external_monitor_example().get_id(), Some("abcde4".to_string()));
        assert_eq!(expression_monitor_example().get_id(), Some("abcde5".to_string()));
    }

    #[test]
    fn monitor_name() {
        assert_eq!(host_monitor_example().get_name(), "Monitor custom.foo.bar".to_string());
        assert_eq!(connectivity_monitor_example().get_name(), "connectivity".to_string());
        assert_eq!(service_monitor_example().get_name(), "Service count".to_string());
        assert_eq!(external_monitor_example().get_name(), "Example external monitor".to_string());
        assert_eq!(
            expression_monitor_example().get_name(),
            "Example expression monitor".to_string()
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

    #[test]
    fn test_monitor_types() {
        let test_cases = [
            (MonitorType::Connectivity, "connectivity"),
            (MonitorType::Host, "host"),
            (MonitorType::Service, "service"),
            (MonitorType::External, "external"),
            (MonitorType::Check, "check"),
            (MonitorType::Expression, "expression"),
        ];
        for &(monitor_type, type_str) in &test_cases {
            let str_value = serde_json::Value::String(type_str.to_string());
            assert_eq!(monitor_type, serde_json::from_value(str_value.clone()).unwrap());
            assert_eq!(str_value, serde_json::to_value(monitor_type).unwrap());
            assert_eq!(str_value, format!("{}", monitor_type).as_str());
        }
    }

    #[test]
    fn test_operators() {
        let test_cases = [(Operator::GreaterThan, ">"), (Operator::LessThan, "<")];
        for &(operator, operator_str) in &test_cases {
            let str_value = serde_json::Value::String(operator_str.to_string());
            assert_eq!(operator, serde_json::from_value(str_value.clone()).unwrap());
            assert_eq!(str_value, serde_json::to_value(operator).unwrap());
            assert_eq!(str_value, format!("{}", operator).as_str());
        }
    }

    #[test]
    fn external_monitor_methods() {
        let test_cases = [
            (ExternalMethod::Get, "GET"),
            (ExternalMethod::Post, "POST"),
            (ExternalMethod::Put, "PUT"),
            (ExternalMethod::Delete, "DELETE"),
        ];
        for &(method, method_str) in &test_cases {
            let str_value = serde_json::Value::String(method_str.to_string());
            assert_eq!(method, serde_json::from_value(str_value.clone()).unwrap());
            assert_eq!(str_value, serde_json::to_value(method).unwrap());
            assert_eq!(str_value, format!("{}", method).as_str());
        }
    }
}

#[derive(Deserialize)]
struct ListMonitorsResponse {
    monitors: Vec<Monitor>,
}

impl client::Client {
    /// Fetches all the monitors.
    ///
    /// See https://mackerel.io/api-docs/entry/monitors#get.
    pub fn list_monitors(&self) -> Result<Vec<Monitor>> {
        self.request(
            Get,
            "/api/v0/monitors",
            vec![],
            client::empty_body(),
            |res: ListMonitorsResponse| res.monitors,
        )
    }

    /// Registers a new monitor.
    ///
    /// See https://mackerel.io/api-docs/entry/monitors#create.
    pub fn create_monitor(&self, monitor: Monitor) -> Result<Monitor> {
        self.request(Post, "/api/v0/monitors", vec![], Some(monitor), |monitor| monitor)
    }

    /// Updates a monitor.
    ///
    /// See https://mackerel.io/api-docs/entry/monitors#update.
    pub fn update_monitor(&self, monitor: Monitor) -> Result<Monitor> {
        let monitor_id: String = monitor.get_id().ok_or("specify the id to update a monitor")?;
        self.request(
            Put,
            format!("/api/v0/monitors/{}", monitor_id),
            vec![],
            Some(monitor),
            |monitor| monitor,
        )
    }

    /// Deletes a monitor.
    ///
    /// See https://mackerel.io/api-docs/entry/monitors#delete.
    pub fn delete_monitor(&self, monitor_id: String) -> Result<Monitor> {
        self.request(
            Delete,
            format!("/api/v0/monitors/{}", monitor_id),
            vec![],
            client::empty_body(),
            |monitor| monitor,
        )
    }
}
