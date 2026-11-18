use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::host::HostId;
use crate::monitor::{MonitorId, MonitorType};

/// An alert entity
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

impl AlertStatus {
    pub(crate) fn critical() -> Self {
        Self::Critical
    }

    pub(crate) fn is_critical(&self) -> bool {
        *self == Self::Critical
    }
}

/// An alert log entity
pub type AlertLog = Entity<AlertLogValue>;

/// An alert log id
pub type AlertLogId = Id<AlertLogValue>;

/// An alert log value
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AlertLogValue {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub status: AlertStatus,
    pub trigger: String,
    #[builder(default, setter(strip_option))]
    pub monitor_id: Option<MonitorId>,
    #[builder(default, setter(strip_option))]
    pub target_value: Option<f64>,
    #[builder(default, setter(strip_option))]
    pub status_detail: Option<AlertLogStatusDetail>,
}

/// An alert log status detail
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct AlertLogStatusDetail {
    #[serde(rename = "type")]
    pub detail_type: String,
    pub detail: AlertLogDetail,
}

/// An alert log detail
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct AlertLogDetail {
    pub message: String,
    #[builder(default)]
    #[serde(default)]
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn alert_example1() -> Alert {
        Alert::builder()
            .id("alert1")
            .value(
                AlertValue::builder()
                    .status(AlertStatus::Critical)
                    .monitor_id("monitor1")
                    .monitor_type(MonitorType::Connectivity)
                    .host_id("host1")
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
            "id": "alert1",
            "status": "CRITICAL",
            "monitorId": "monitor1",
            "type": "connectivity",
            "hostId": "host1",
            "message": "alert message",
            "reason": "alert close reason",
            "openedAt": 1690000000,
            "closedAt": 1700000000,
        })
    }

    fn alert_example2() -> Alert {
        Alert::builder()
            .id("alert2")
            .value(
                AlertValue::builder()
                    .status(AlertStatus::Warning)
                    .monitor_id("monitor2")
                    .monitor_type(MonitorType::Host)
                    .host_id("host2")
                    .value(25.0)
                    .opened_at(DateTime::from_timestamp(1690000000, 0).unwrap())
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "alert2",
            "status": "WARNING",
            "monitorId": "monitor2",
            "type": "host",
            "hostId": "host2",
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

    fn alert_log_example1() -> AlertLog {
        AlertLog::builder()
            .id("log1")
            .value(
                AlertLogValue::builder()
                    .created_at(DateTime::from_timestamp(1735290407, 0).unwrap())
                    .status(AlertStatus::Warning)
                    .trigger("monitoring")
                    .monitor_id("monitor1")
                    .status_detail(
                        AlertLogStatusDetail::builder()
                            .detail_type("check")
                            .detail(
                                AlertLogDetail::builder()
                                    .message("Uptime WARNING: 0 day(s) 0 hour(s) 6 minute(s)")
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build()
    }

    fn alert_log_json_example1() -> serde_json::Value {
        json!({
            "id": "log1",
            "createdAt": 1735290407,
            "status": "WARNING",
            "trigger": "monitoring",
            "monitorId": "monitor1",
            "statusDetail": {
                "type": "check",
                "detail": {
                    "message": "Uptime WARNING: 0 day(s) 0 hour(s) 6 minute(s)",
                    "memo": "",
                },
            },
        })
    }

    fn alert_log_example2() -> AlertLog {
        AlertLog::builder()
            .id("log2")
            .value(
                AlertLogValue::builder()
                    .created_at(DateTime::from_timestamp(1735290407, 0).unwrap())
                    .status(AlertStatus::Critical)
                    .trigger("monitoring")
                    .monitor_id("monitor2")
                    .target_value(95.0)
                    .build(),
            )
            .build()
    }

    fn alert_log_json_example2() -> serde_json::Value {
        json!({
            "id": "log2",
            "createdAt": 1735290407,
            "status": "CRITICAL",
            "trigger": "monitoring",
            "monitorId": "monitor2",
            "targetValue": 95.0,
        })
    }

    #[rstest]
    #[case(alert_log_example1(), alert_log_json_example1())]
    #[case(alert_log_example2(), alert_log_json_example2())]
    fn test_alert_log_json(#[case] alert_log: AlertLog, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&alert_log).unwrap(), json);
        assert_eq!(alert_log, serde_json::from_value(json).unwrap());
    }

    #[test]
    fn test_alert_log_json_null_status_detail() {
        let mut json = alert_log_json_example2();
        json["statusDetail"] = json!(null);
        assert_eq!(
            alert_log_example2(),
            serde_json::from_value::<AlertLog>(json).unwrap()
        );
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

impl Client {
    /// Fetches open alerts.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#list>.
    pub async fn list_open_alerts(
        &self,
        cursor_opt: Option<impl Into<AlertId>>,
        limit: u8,
    ) -> Result<(Vec<Alert>, Option<AlertId>)> {
        self.list_alerts("", cursor_opt, limit).await
    }

    /// Fetches all (open or closed) alerts.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#list>.
    pub async fn list_all_alerts(
        &self,
        cursor_opt: Option<impl Into<AlertId>>,
        limit: u8,
    ) -> Result<(Vec<Alert>, Option<AlertId>)> {
        self.list_alerts("true", cursor_opt, limit).await
    }

    async fn list_alerts(
        &self,
        with_closed: &str,
        cursor_opt: Option<impl Into<AlertId>>,
        limit: u8,
    ) -> Result<(Vec<Alert>, Option<AlertId>)> {
        self.request(
            Method::GET,
            "/api/v0/alerts",
            query_params! {
                withClosed = with_closed,
                nextId = cursor_opt.map(Into::into).as_deref().unwrap_or_default(),
                limit = limit.to_string(),
            },
            request_body![],
            response_body! { alerts: Vec<Alert>, nextId: Option<AlertId> },
        )
        .await
    }

    /// Gets an alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#get>.
    pub async fn get_alert(&self, alert_id: impl Into<AlertId>) -> Result<Alert> {
        self.request(
            Method::GET,
            format!("/api/v0/alerts/{}", alert_id.into()),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Updates an alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#update>.
    pub async fn update_alert(
        &self,
        alert_id: impl Into<AlertId>,
        memo: impl AsRef<str>,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/alerts/{}", alert_id.into()),
            query_params![],
            request_body! { memo: String = memo.as_ref().to_owned() },
            response_body!(),
        )
        .await
    }

    /// Closes the specified alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#close>.
    pub async fn close_alert(
        &self,
        alert_id: impl Into<AlertId>,
        reason: impl AsRef<str>,
    ) -> Result<Alert> {
        self.request(
            Method::POST,
            format!("/api/v0/alerts/{}/close", alert_id.into()),
            query_params![],
            request_body! { reason: String = reason.as_ref().to_owned() },
            response_body!(..),
        )
        .await
    }

    /// Fetches logs of the specified alert.
    ///
    /// See <https://mackerel.io/api-docs/entry/alerts#logs>.
    pub async fn list_alert_logs(
        &self,
        alert_id: impl Into<AlertId>,
        cursor_opt: Option<impl Into<AlertLogId>>,
        limit: u8,
    ) -> Result<(Vec<AlertLog>, Option<AlertLogId>)> {
        self.request(
            Method::GET,
            format!("/api/v0/alerts/{}/logs", alert_id.into()),
            query_params! {
                nextId = cursor_opt.map(Into::into).as_deref().unwrap_or_default(),
                limit = limit.to_string(),
            },
            request_body![],
            response_body! { logs: Vec<AlertLog>, nextId: Option<AlertLogId> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::alert::*;
    use crate::tests::*;

    fn value_example() -> AlertValue {
        AlertValue::builder()
            .status(AlertStatus::Ok)
            .monitor_type(MonitorType::Host)
            .reason("alert close reason")
            .opened_at(DateTime::from_timestamp(1698890400, 0).unwrap())
            .closed_at(DateTime::from_timestamp(1698894000, 0).unwrap())
            .build()
    }

    fn entity_example() -> Alert {
        Alert {
            id: AlertId::from("alert1"),
            value: value_example(),
        }
    }

    fn entity_json_example() -> serde_json::Value {
        json!({
            "id": "alert1",
            "status": "OK",
            "type": "host",
            "reason": "alert close reason",
            "openedAt": 1698890400,
            "closedAt": 1698894000,
        })
    }

    #[tokio::test]
    async fn list_open_alerts() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/alerts",
            query_params = "limit=10",
            response = json!({ "alerts": [] }),
        };
        assert_eq!(
            test_client!(server)
                .list_open_alerts(None::<AlertId>, 10)
                .await,
            Ok((vec![], None)),
        );
    }

    #[tokio::test]
    async fn list_all_alerts() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/alerts",
            query_params = "withClosed=true&limit=1&nextId=alert1",
            response = json!({
                "alerts": [entity_json_example()],
                "nextId": "alert2",
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_all_alerts(Some("alert1"), 1)
                .await,
            Ok((vec![entity_example()], Some("alert2".into()))),
        );
        assert_eq!(
            test_client!(server)
                .list_all_alerts(Some(AlertId::from("alert1")), 1)
                .await,
            Ok((vec![entity_example()], Some(AlertId::from("alert2")))),
        );
    }

    #[tokio::test]
    async fn get_alert() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/alerts/alert1",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).get_alert("alert1").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .get_alert(AlertId::from("alert1"))
                .await,
            Ok(entity_example()),
        );
    }

    #[tokio::test]
    async fn update_alert() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/alerts/alert1",
            request = json!({ "memo": "alert memo" }),
            response = json!({ "id": "alert1", "memo": "alert memo" }),
        };
        assert_eq!(
            test_client!(server)
                .update_alert("alert1", "alert memo")
                .await,
            Ok(()),
        );
        assert_eq!(
            test_client!(server)
                .update_alert(AlertId::from("alert1"), String::from("alert memo"))
                .await,
            Ok(()),
        );
    }

    #[tokio::test]
    async fn close_alert() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/alerts/alert1/close",
            request = json!({ "reason": "alert close reason" }),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .close_alert("alert1", "alert close reason")
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .close_alert(AlertId::from("alert1"), String::from("alert close reason"))
                .await,
            Ok(entity_example()),
        );
    }

    #[tokio::test]
    async fn list_alert_logs() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/alerts/alert1/logs",
            query_params = "limit=1&nextId=log1",
            response = json!({
                "logs": [{
                    "id": "log2",
                    "createdAt": 1698890400,
                    "status": "OK",
                    "trigger": "manual",
                }],
                "nextId": "log3",
            }),
        };
        let alert_log = AlertLog {
            id: AlertLogId::from("log2"),
            value: AlertLogValue::builder()
                .created_at(DateTime::from_timestamp(1698890400, 0).unwrap())
                .status(AlertStatus::Ok)
                .trigger("manual")
                .build(),
        };
        assert_eq!(
            test_client!(server)
                .list_alert_logs("alert1", Some("log1"), 1)
                .await,
            Ok((vec![alert_log.clone()], Some("log3".into()))),
        );
        assert_eq!(
            test_client!(server)
                .list_alert_logs(AlertId::from("alert1"), Some(AlertLogId::from("log1")), 1)
                .await,
            Ok((vec![alert_log], Some(AlertLogId::from("log3")))),
        );
    }
}
