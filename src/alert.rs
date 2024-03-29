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
                withClosed = *with_closed,
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
            format_url!("/api/v0/alerts/{}", alert_id),
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
            format_url!("/api/v0/alerts/{}", alert_id),
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
            format_url!("/api/v0/alerts/{}/close", alert_id),
            query_params![],
            request_body! { reason: String = reason.as_ref().to_owned() },
            response_body!(..),
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

    #[async_std::test]
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

    #[async_std::test]
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

    #[async_std::test]
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

    #[async_std::test]
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

    #[async_std::test]
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
}
