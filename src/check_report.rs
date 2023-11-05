use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::alert::AlertStatus;
use crate::client::*;
use crate::error::Result;
use crate::host::HostId;

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
}

#[cfg(test)]
mod client_tests {
    use chrono::DateTime;

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
}
