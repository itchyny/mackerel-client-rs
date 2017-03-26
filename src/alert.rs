use std::collections::HashMap;
use hyper::method::Method::*;
use client;
use errors::*;
use monitor::MonitorType;

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
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlertStatus {
    Ok,
    Critical,
    Warning,
    Unknown,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use alert::*;

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
        serde_json::from_str(r##"
            {
                "id": "abcde0",
                "status": "CRITICAL",
                "monitorId": "abcde2",
                "type": "connectivity",
                "hostId": "abcde1"
            }
        "##)
            .unwrap()
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
        serde_json::from_str(r##"
            {
                "id": "abcde0",
                "status": "WARNING",
                "monitorId": "abcde2",
                "type": "host",
                "hostId": "abcde1",
                "value": 25.0
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_alert() {
        assert_eq!(json_example1(),
                   serde_json::to_value(&alert_example1()).unwrap());
        assert_eq!(json_example2(),
                   serde_json::to_value(&alert_example2()).unwrap());
    }

    #[test]
    fn deserialize_alert() {
        assert_eq!(alert_example1(),
                   serde_json::from_value(json_example1()).unwrap());
        assert_eq!(alert_example2(),
                   serde_json::from_value(json_example2()).unwrap());
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
        self.request(Get,
                     "/api/v0/alerts",
                     vec![],
                     client::empty_body(),
                     |res: ListAlertsResponse| res.alerts)
    }

    /// Closes the specified alert.
    ///
    /// See https://mackerel.io/api-docs/entry/alerts#close.
    pub fn close_alert(&self, alert_id: String, reason: &str) -> Result<Alert> {
        let body: HashMap<&str, &str> = [("reason", reason)].iter().cloned().collect();
        self.request(Post,
                     format!("/api/v0/alerts/{}/close", alert_id),
                     vec![],
                     Some(body),
                     |alert| alert)
    }
}
