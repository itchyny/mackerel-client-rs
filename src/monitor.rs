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

/// Monitor types
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MonitorType {
    Connectivity,
    Host,
    Service,
    External,
    Check,
    Expression,
}

/// Monitor operators
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
}

/// HTTP methods for external http monitors
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalMethod {
    Get,
    Post,
    Put,
    DELETE,
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
        serde_json::from_str(r##"
            {
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
            }
        "##)
            .unwrap()
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
        serde_json::from_str(r##"
            {
                "type": "connectivity",
                "id": "abcde2",
                "name": "connectivity",
                "memo": "Monitor memo",
                "isMute": false
            }
        "##)
            .unwrap()
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
        serde_json::from_str(r##"
            {
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
            }
        "##)
            .unwrap()
    }

    fn external_monitor_example() -> Monitor {
        Monitor::External {
            id: Some("abcde4".to_string()),
            name: "Example external monitor".to_string(),
            memo: Some("Monitor memo".to_string()),
            method: Some(ExternalMethod::Get),
            url: "https://example.com".to_string(),
            request_body: Some("Request Body".to_string()),
            headers: Some(vec![ExternalHeader {
                                   name: "Cache-Control".to_string(),
                                   value: "no-cache".to_string(),
                               }]),
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
        serde_json::from_str(r##"
            {
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
            }
        "##)
            .unwrap()
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
        serde_json::from_str(r##"
            {
                "type": "expression",
                "id": "abcde5",
                "name": "Example expression monitor",
                "memo": "Monitor memo",
                "expression": "min(role(\"foo:bar\", \"custom.foo.bar\"))",
                "operator": "<",
                "warning": 10.0,
                "critical": 20.0,
                "isMute": false
            }
        "##)
            .unwrap()
    }

    fn monitor_examples() -> Vec<(Monitor, serde_json::Value)> {
        vec![(host_monitor_example(), host_monitor_json_example()),
             (connectivity_monitor_example(), connectivity_monitor_json_example()),
             (service_monitor_example(), service_monitor_json_example()),
             (external_monitor_example(), external_monitor_json_example()),
             (expression_monitor_example(), expression_monitor_json_example())]
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
}
