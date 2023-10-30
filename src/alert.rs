use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::host::HostId;
use crate::monitor::{MonitorId, MonitorType};
use crate::response;

/// An alert
pub type Alert = Entity<AlertValue>;

/// An alert id
pub type AlertId = Id<AlertValue>;

/// An alert value
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AlertValue {
    pub status: AlertStatus,
    #[builder(default, setter(strip_option))]
    pub monitor_id: Option<MonitorId>,
    #[serde(rename = "type")]
    pub monitor_type: MonitorType,
    #[builder(default, setter(strip_option))]
    pub host_id: Option<HostId>,
    #[builder(default, setter(strip_option))]
    pub value: Option<f64>,
    #[builder(default, setter(strip_option))]
    pub message: Option<String>,
    #[builder(default, setter(strip_option))]
    pub reason: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub opened_at: DateTime<Utc>,
    #[builder(default, setter(strip_option))]
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub closed_at: Option<DateTime<Utc>>,
}

/// Alert status
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum AlertStatus {
    Ok,
    Critical,
    Warning,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn alert_example1() -> Alert {
        Alert::builder()
            .id("abcde1")
            .value(
                AlertValue::builder()
                    .status(AlertStatus::Critical)
                    .monitor_id("abcde2")
                    .monitor_type(MonitorType::Connectivity)
                    .host_id("abcde3")
                    .message("alert message")
                    .reason("alert close reason")
                    .opened_at(DateTime::from_timestamp(1690000000, 0).unwrap())
                    .closed_at(DateTime::from_timestamp(1700000000, 0).unwrap())
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "status": "CRITICAL",
            "monitorId": "abcde2",
            "type": "connectivity",
            "hostId": "abcde3",
            "message": "alert message",
            "reason": "alert close reason",
            "openedAt": 1690000000,
            "closedAt": 1700000000,
        })
    }

    fn alert_example2() -> Alert {
        Alert::builder()
            .id("abcde1")
            .value(
                AlertValue::builder()
                    .status(AlertStatus::Warning)
                    .monitor_id("abcde2")
                    .monitor_type(MonitorType::Host)
                    .host_id("abcde3")
                    .value(25.0)
                    .opened_at(DateTime::from_timestamp(1690000000, 0).unwrap())
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "status": "WARNING",
            "monitorId": "abcde2",
            "type": "host",
            "hostId": "abcde3",
            "value": 25.0,
            "openedAt": 1690000000,
        })
    }

    #[rstest]
    #[case(alert_example1(), json_example1())]
    #[case(alert_example2(), json_example2())]
    fn test_alert_json(#[case] alert: Alert, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&alert).unwrap(), json);
        assert_eq!(alert, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(AlertStatus::Ok, "OK")]
    #[case(AlertStatus::Critical, "CRITICAL")]
    #[case(AlertStatus::Warning, "WARNING")]
    #[case(AlertStatus::Unknown, "UNKNOWN")]
    fn test_alert_status(#[case] alert_status: AlertStatus, #[case] alert_status_str: &str) {
        assert_eq!(alert_status.to_string(), alert_status_str);
        assert_eq!(alert_status, alert_status_str.parse().unwrap());
        assert_eq!(
            alert_status,
            serde_json::from_value(alert_status_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(alert_status).unwrap(),
            alert_status_str
        );
    }
}

impl client::Client {
    /// Fetches open alerts.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#get>.
    pub async fn list_open_alerts(
        &self,
        cursor_opt: Option<AlertId>,
        limit: usize,
    ) -> Result<(Vec<Alert>, Option<AlertId>)> {
        self.list_alerts("false", cursor_opt, limit).await
    }

    /// Fetches closed alerts.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#get>.
    pub async fn list_closed_alerts(
        &self,
        cursor_opt: Option<AlertId>,
        limit: usize,
    ) -> Result<(Vec<Alert>, Option<AlertId>)> {
        self.list_alerts("true", cursor_opt, limit).await
    }

    async fn list_alerts(
        &self,
        with_closed: &str,
        cursor_opt: Option<AlertId>,
        limit: usize,
    ) -> Result<(Vec<Alert>, Option<AlertId>)> {
        self.request(
            Method::GET,
            "/api/v0/alerts",
            vec![
                ("withClosed", vec![with_closed]),
                ("nextId", cursor_opt.as_deref().into_iter().collect()),
                ("limit", vec![limit.to_string().as_str()]),
            ],
            client::empty_body(),
            response! { alerts: Vec<Alert>, nextId: Option<AlertId> },
        )
        .await
    }

    /// Gets an alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#get>.
    pub async fn get_alert(&self, alert_id: AlertId) -> Result<Alert> {
        self.request(
            Method::GET,
            format!("/api/v0/alerts/{}", alert_id),
            vec![],
            client::empty_body(),
            |alert| alert,
        )
        .await
    }

    /// Updates an alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#update>.
    pub async fn update_alert(&self, alert_id: AlertId, memo: String) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/alerts/{}", alert_id),
            vec![],
            Some(HashMap::from([("memo", memo)])),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Closes the specified alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#close>.
    pub async fn close_alert(&self, alert_id: AlertId, reason: String) -> Result<Alert> {
        self.request(
            Method::POST,
            format!("/api/v0/alerts/{}/close", alert_id),
            vec![],
            Some(HashMap::from([("reason", reason)])),
            |alert| alert,
        )
        .await
    }
}
