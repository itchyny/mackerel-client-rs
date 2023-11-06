use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::host::HostId;
use crate::service::ServiceName;

/// A host metric value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HostMetricValue {
    pub host_id: HostId,
    pub name: String,
    #[serde(flatten)]
    pub value: MetricValue,
}

/// A service metric value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct ServiceMetricValue {
    pub name: String,
    #[serde(flatten)]
    pub value: MetricValue,
}

/// A metric value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct MetricValue {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn host_metric_example1() -> HostMetricValue {
        HostMetricValue::builder()
            .host_id("host1")
            .name("loadavg.loadavg1")
            .value(
                MetricValue::builder()
                    .time(DateTime::from_timestamp(1700000000, 0).unwrap())
                    .value(1.2)
                    .build(),
            )
            .build()
    }

    fn host_metric_json_example1() -> serde_json::Value {
        json!({
            "hostId": "host1",
            "name": "loadavg.loadavg1",
            "time": 1700000000,
            "value": 1.2,
        })
    }

    fn service_metric_example1() -> ServiceMetricValue {
        ServiceMetricValue::builder()
            .name("custom.metric.name")
            .value(
                MetricValue::builder()
                    .time(DateTime::from_timestamp(1700000000, 0).unwrap())
                    .value(1.3)
                    .build(),
            )
            .build()
    }

    fn service_metric_json_example1() -> serde_json::Value {
        json!({
            "name": "custom.metric.name",
            "time": 1700000000,
            "value": 1.3,
        })
    }

    #[rstest]
    #[case(host_metric_example1(), host_metric_json_example1())]
    #[case(service_metric_example1(), service_metric_json_example1())]
    fn test_metric_value<
        MetricValue: PartialEq + std::fmt::Debug + serde::ser::Serialize + serde::de::DeserializeOwned,
    >(
        #[case] metric_value: MetricValue,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&metric_value).unwrap(), json);
        assert_eq!(metric_value, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Posts host metric values.
    ///
    /// See <https://mackerel.io/api-docs/entry/host-metrics#post>.
    pub async fn post_host_metric_values(
        &self,
        host_metric_values: impl IntoIterator<Item = HostMetricValue>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/tsdb",
            query_params![],
            request_body!(host_metric_values.into_iter().collect::<Vec<_>>()),
            response_body!(),
        )
        .await
    }

    /// Fetches host metric value.
    ///
    /// See <https://mackerel.io/api-docs/entry/host-metrics#get>.
    pub async fn list_host_metric_values(
        &self,
        host_id: impl Into<HostId>,
        metric_name: impl AsRef<str>,
        from: impl Into<DateTime<Utc>>,
        to: impl Into<DateTime<Utc>>,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format_url!("/api/v0/hosts/{}/metrics", host_id),
            query_params! {
                name = metric_name.as_ref(),
                from = from.into().timestamp().to_string(),
                to = to.into().timestamp().to_string(),
            },
            request_body![],
            response_body! { metrics: Vec<MetricValue> },
        )
        .await
    }

    /// Posts service metric values.
    ///
    /// See <https://mackerel.io/api-docs/entry/service-metrics#post>.
    pub async fn post_service_metric_values(
        &self,
        service_name: impl Into<ServiceName>,
        service_metric_values: impl IntoIterator<Item = ServiceMetricValue>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            format_url!("/api/v0/services/{}/tsdb", service_name),
            query_params![],
            request_body!(service_metric_values.into_iter().collect::<Vec<_>>()),
            response_body!(),
        )
        .await
    }

    /// Fetches service metric values.
    ///
    /// See <https://mackerel.io/api-docs/entry/service-metrics#get>.
    pub async fn list_service_metric_values(
        &self,
        service_name: impl Into<ServiceName>,
        metric_name: impl AsRef<str>,
        from: impl Into<DateTime<Utc>>,
        to: impl Into<DateTime<Utc>>,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format_url!("/api/v0/services/{}/metrics", service_name),
            query_params! {
                name = metric_name.as_ref(),
                from = from.into().timestamp().to_string(),
                to = to.into().timestamp().to_string(),
            },
            request_body![],
            response_body! { metrics: Vec<MetricValue> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use crate::metric::*;
    use crate::tests::*;

    #[async_std::test]
    async fn post_host_metric_values() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/tsdb",
            request = json!([
                {
                    "hostId": "host0",
                    "name": "loadavg1",
                    "time": 1698894000,
                    "value": 1.0,
                },
                {
                    "hostId": "host0",
                    "name": "loadavg5",
                    "time": 1698894000,
                    "value": 1.1,
                },
                {
                    "hostId": "host0",
                    "name": "loadavg15",
                    "time": 1698894000,
                    "value": 1.2,
                },
            ]),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .post_host_metric_values([
                    HostMetricValue {
                        host_id: "host0".into(),
                        name: "loadavg1".to_owned(),
                        value: MetricValue {
                            time: DateTime::from_timestamp(1698894000, 0).unwrap(),
                            value: 1.0,
                        },
                    },
                    HostMetricValue {
                        host_id: "host0".into(),
                        name: "loadavg5".to_owned(),
                        value: MetricValue {
                            time: DateTime::from_timestamp(1698894000, 0).unwrap(),
                            value: 1.1,
                        },
                    },
                    HostMetricValue {
                        host_id: "host0".into(),
                        name: "loadavg15".to_owned(),
                        value: MetricValue {
                            time: DateTime::from_timestamp(1698894000, 0).unwrap(),
                            value: 1.2,
                        },
                    },
                ])
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn list_host_metric_values() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts/host0/metrics",
            query_params = "name=loadavg5&from=1699999860&to=1700000000",
            response = json!({
                "metrics": [
                    { "time": 1699999860, "value": 1.0 },
                    { "time": 1699999920, "value": 1.1 },
                    { "time": 1699999980, "value": 1.2 },
                ],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_host_metric_values(
                    "host0",
                    "loadavg5",
                    DateTime::from_timestamp(1699999860, 0).unwrap(),
                    DateTime::from_timestamp(1700000000, 0).unwrap(),
                )
                .await,
            Ok(vec![
                MetricValue {
                    time: DateTime::from_timestamp(1699999860, 0).unwrap(),
                    value: 1.0,
                },
                MetricValue {
                    time: DateTime::from_timestamp(1699999920, 0).unwrap(),
                    value: 1.1,
                },
                MetricValue {
                    time: DateTime::from_timestamp(1699999980, 0).unwrap(),
                    value: 1.2,
                },
            ]),
        );
    }

    #[async_std::test]
    async fn post_service_metric_values() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/services/service0/tsdb",
            request = json!([
                { "name": "custom.metric0", "time": 1698894000, "value": 1.0 },
                { "name": "custom.metric1", "time": 1698894000, "value": 1.1 },
                { "name": "custom.metric2", "time": 1698894000, "value": 1.2 },
            ]),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .post_service_metric_values(
                    "service0",
                    [
                        ServiceMetricValue {
                            name: "custom.metric0".to_owned(),
                            value: MetricValue {
                                time: DateTime::from_timestamp(1698894000, 0).unwrap(),
                                value: 1.0,
                            },
                        },
                        ServiceMetricValue {
                            name: "custom.metric1".to_owned(),
                            value: MetricValue {
                                time: DateTime::from_timestamp(1698894000, 0).unwrap(),
                                value: 1.1,
                            },
                        },
                        ServiceMetricValue {
                            name: "custom.metric2".to_owned(),
                            value: MetricValue {
                                time: DateTime::from_timestamp(1698894000, 0).unwrap(),
                                value: 1.2,
                            },
                        }
                    ]
                )
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn list_service_metric_values() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/metrics",
            query_params = "name=custom.metric&from=1699999860&to=1700000000",
            response = json!({
                "metrics": [
                    { "time": 1699999860, "value": 1.0 },
                    { "time": 1699999920, "value": 1.1 },
                    { "time": 1699999980, "value": 1.2 },
                ],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_service_metric_values(
                    "service0",
                    "custom.metric",
                    DateTime::from_timestamp(1699999860, 0).unwrap(),
                    DateTime::from_timestamp(1700000000, 0).unwrap(),
                )
                .await,
            Ok(vec![
                MetricValue {
                    time: DateTime::from_timestamp(1699999860, 0).unwrap(),
                    value: 1.0,
                },
                MetricValue {
                    time: DateTime::from_timestamp(1699999920, 0).unwrap(),
                    value: 1.1,
                },
                MetricValue {
                    time: DateTime::from_timestamp(1699999980, 0).unwrap(),
                    value: 1.2,
                },
            ]),
        );
    }
}
