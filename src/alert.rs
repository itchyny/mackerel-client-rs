use crate::{client, errors::*, monitor::MonitorType};
use reqwest::Method;
use std::collections::HashMap;
use std::fmt;

/// An alert
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: String,
    pub status: AlertStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_id: Option<String>,
    #[serde(rename = "type")]
    pub monitor_type: MonitorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    // pub openedAt: DateTime, // TODO
    // pub closedAt: Option<DateTime>, // TODO
}

/// Alert statuses
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertStatus {
    Ok,
    Critical,
    Warning,
    Unknown,
}

impl fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlertStatus::Ok => write!(f, "OK"),
            AlertStatus::Critical => write!(f, "CRITICAL"),
            AlertStatus::Warning => write!(f, "WARNING"),
            AlertStatus::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alert::*;
    use serde_json;

    fn alert_example1() -> Alert {
        Alert {
            id: "abcde0".to_string(),
            status: AlertStatus::Critical,
            monitor_id: Some("abcde2".to_string()),
            monitor_type: MonitorType::Connectivity,
            host_id: Some("abcde1".to_string()),
            value: None,
            message: None,
            reason: None,
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde0",
            "status": "CRITICAL",
            "monitorId": "abcde2",
            "type": "connectivity",
            "hostId": "abcde1"
        })
    }

    fn alert_example2() -> Alert {
        Alert {
            id: "abcde0".to_string(),
            status: AlertStatus::Warning,
            monitor_id: Some("abcde2".to_string()),
            monitor_type: MonitorType::Host,
            host_id: Some("abcde1".to_string()),
            value: Some(25.0),
            message: None,
            reason: None,
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde0",
            "status": "WARNING",
            "monitorId": "abcde2",
            "type": "host",
            "hostId": "abcde1",
            "value": 25.0
        })
    }

    #[test]
    fn serialize_alert() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&alert_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&alert_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_alert() {
        assert_eq!(
            alert_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            alert_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
    }

    #[test]
    fn alert_statuses() {
        let test_cases = [
            (AlertStatus::Ok, "OK"),
            (AlertStatus::Critical, "CRITICAL"),
            (AlertStatus::Warning, "WARNING"),
            (AlertStatus::Unknown, "UNKNOWN"),
        ];
        for &(status, status_str) in &test_cases {
            let str_value = serde_json::Value::String(status_str.to_string());
            assert_eq!(status, serde_json::from_value(str_value.clone()).unwrap());
            assert_eq!(str_value, serde_json::to_value(status).unwrap());
            assert_eq!(str_value, format!("{}", status).as_str());
        }
    }
}

#[derive(Deserialize)]
struct ListAlertsResponse {
    alerts: Vec<Alert>,
}

impl client::Client {
    /// Fetches all the open alerts.
    ///
    /// See https://mackerel.io/api-docs/entry/alerts#get.
    pub fn list_alerts(&self) -> Result<Vec<Alert>> {
        self.request(
            Method::GET,
            "/api/v0/alerts",
            vec![],
            client::empty_body(),
            |res: ListAlertsResponse| res.alerts,
        )
    }

    /// Closes the specified alert.
    ///
    /// See https://mackerel.io/api-docs/entry/alerts#close.
    pub fn close_alert(&self, alert_id: String, reason: &str) -> Result<Alert> {
        let body: HashMap<&str, &str> = [("reason", reason)].iter().cloned().collect();
        self.request(
            Method::POST,
            format!("/api/v0/alerts/{}/close", alert_id),
            vec![],
            Some(body),
            |alert| alert,
        )
    }
}
