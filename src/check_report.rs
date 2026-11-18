use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::alert::AlertStatus;
use crate::client::*;
use crate::error::Result;
use crate::host::HostId;
use crate::monitor::MonitorId;

/// A check report
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct CheckReport {
    pub name: String,
    #[builder(default)]
    pub message: String,
    pub source: CheckSource,
    pub status: AlertStatus,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub occurred_at: DateTime<Utc>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_check_attempts: Option<u64>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CheckSource {
    #[serde(rename_all = "camelCase")]
    Host { host_id: HostId },
}

/// A check monitor
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct CheckMonitor {
    pub id: MonitorId,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn check_report_example1() -> CheckReport {
        CheckReport::builder()
            .name("ExampleCheckReport1")
            .source(CheckSource::Host {
                host_id: "host0".into(),
            })
            .status(AlertStatus::Ok)
            .occurred_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "name": "ExampleCheckReport1",
            "message": "",
            "source": {
                "type": "host",
                "hostId": "host0",
            },
            "status": "OK",
            "occurredAt": 1700000000,
        })
    }

    fn check_report_example2() -> CheckReport {
        CheckReport::builder()
            .name("ExampleCheckReport2")
            .message("example check message")
            .source(CheckSource::Host {
                host_id: "host0".into(),
            })
            .status(AlertStatus::Critical)
            .occurred_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .notification_interval(60)
            .max_check_attempts(5)
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "name": "ExampleCheckReport2",
            "message": "example check message",
            "source": {
                "type": "host",
                "hostId": "host0",
            },
            "status": "CRITICAL",
            "occurredAt": 1700000000,
            "notificationInterval": 60,
            "maxCheckAttempts": 5,
        })
    }

    #[rstest]
    #[case(check_report_example1(), json_example1())]
    #[case(check_report_example2(), json_example2())]
    fn test_check_report_json(#[case] check_report: CheckReport, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&check_report).unwrap(), json);
        assert_eq!(check_report, serde_json::from_value(json).unwrap());
    }

    #[test]
    fn test_check_monitor_json() {
        let check_monitor = CheckMonitor::builder()
            .id("monitor0")
            .name("ExampleCheckMonitor")
            .build();
        let json = json!({ "id": "monitor0", "name": "ExampleCheckMonitor" });
        assert_eq!(serde_json::to_value(&check_monitor).unwrap(), json);
        assert_eq!(check_monitor, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Creates a new check report.
    ///
    /// See <https://mackerel.io/api-docs/entry/check-monitoring#post>.
    pub async fn create_check_report(
        &self,
        check_reports: impl IntoIterator<Item = CheckReport>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/monitoring/checks/report",
            query_params![],
            request_body! {
                reports: Vec<CheckReport> = check_reports
                    .into_iter().collect::<Vec<_>>(),
            },
            response_body!(),
        )
        .await
    }

    /// Fetches check monitors.
    ///
    /// See <https://mackerel.io/api-docs/entry/check-monitoring#list>.
    pub async fn list_check_monitors(
        &self,
        cursor_opt: Option<impl Into<MonitorId>>,
        limit: u8,
    ) -> Result<(Vec<CheckMonitor>, Option<MonitorId>)> {
        self.request(
            Method::GET,
            "/api/v0/monitoring/checks",
            query_params! {
                nextId = *cursor_opt.map(Into::into).as_deref().unwrap_or_default(),
                limit = limit.to_string(),
            },
            request_body![],
            response_body! { checks: Vec<CheckMonitor>, nextId: Option<MonitorId> },
        )
        .await
    }

    /// Gets a check monitor.
    ///
    /// See <https://mackerel.io/api-docs/entry/check-monitoring#retrieve>.
    pub async fn get_check_monitor(
        &self,
        monitor_id: impl Into<MonitorId>,
    ) -> Result<CheckMonitor> {
        self.request(
            Method::GET,
            format_url!("/api/v0/monitoring/checks/{}", monitor_id),
            query_params![],
            request_body![],
            response_body! { check: CheckMonitor },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::check_report::*;
    use crate::tests::*;

    #[async_std::test]
    async fn create_check_report() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/monitoring/checks/report",
            request = json!({
                "reports": [
                    {
                        "name": "ExampleCheckReport",
                        "message": "This is an example check message.",
                        "source": {
                            "type": "host",
                            "hostId": "host0",
                        },
                        "status": "WARNING",
                        "occurredAt": 1698890400,
                        "notificationInterval": 60,
                        "maxCheckAttempts": 5,
                    },
                ],
            }),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .create_check_report([CheckReport::builder()
                    .name("ExampleCheckReport")
                    .message("This is an example check message.")
                    .source(CheckSource::Host {
                        host_id: "host0".into(),
                    })
                    .status(AlertStatus::Warning)
                    .occurred_at(DateTime::from_timestamp(1698890400, 0).unwrap())
                    .notification_interval(60)
                    .max_check_attempts(5)
                    .build()])
                .await,
            Ok(())
        );
    }

    fn check_monitor_example() -> CheckMonitor {
        CheckMonitor::builder()
            .id("monitor1")
            .name("ExampleCheckMonitor")
            .build()
    }

    fn check_monitor_json_example() -> serde_json::Value {
        json!({ "id": "monitor1", "name": "ExampleCheckMonitor" })
    }

    #[async_std::test]
    async fn list_check_monitors() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/monitoring/checks",
            query_params = "limit=100",
            response = json!({ "checks": [check_monitor_json_example()] }),
        };
        assert_eq!(
            test_client!(server)
                .list_check_monitors(None::<MonitorId>, 100)
                .await,
            Ok((vec![check_monitor_example()], None)),
        );
    }

    #[async_std::test]
    async fn list_check_monitors_with_cursor() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/monitoring/checks",
            query_params = "limit=1&nextId=monitor0",
            response = json!({
                "checks": [check_monitor_json_example()],
                "nextId": "monitor2",
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_check_monitors(Some("monitor0"), 1)
                .await,
            Ok((vec![check_monitor_example()], Some("monitor2".into()))),
        );
        assert_eq!(
            test_client!(server)
                .list_check_monitors(Some(MonitorId::from("monitor0")), 1)
                .await,
            Ok((
                vec![check_monitor_example()],
                Some(MonitorId::from("monitor2"))
            )),
        );
    }

    #[async_std::test]
    async fn get_check_monitor() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/monitoring/checks/monitor1",
            response = json!({ "check": check_monitor_json_example() }),
        };
        assert_eq!(
            test_client!(server).get_check_monitor("monitor1").await,
            Ok(check_monitor_example()),
        );
        assert_eq!(
            test_client!(server)
                .get_check_monitor(MonitorId::from("monitor1"))
                .await,
            Ok(check_monitor_example()),
        );
    }
}
