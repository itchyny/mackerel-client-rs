use crate::client;
use crate::error::*;
use crate::host::HostId;
use crate::service::ServiceName;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

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
    use crate::metric::*;
    use rstest::rstest;
    use serde_json::json;

    fn host_metric_example1() -> HostMetricValue {
        HostMetricValue::builder()
            .host_id("abcde1")
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
            "hostId": "abcde1",
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

#[derive(Deserialize)]
struct ListMetricValuesResponse {
    metrics: Vec<MetricValue>,
}

impl client::Client {
    /// Posts host metric values.
    ///
    /// See <https://mackerel.io/api-docs/entry/host-metrics#post>.
    pub async fn post_host_metric_values(
        &self,
        host_metric_values: Vec<HostMetricValue>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/tsdb",
            vec![],
            Some(host_metric_values),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Fetches host metric value.
    ///
    /// See <https://mackerel.io/api-docs/entry/host-metrics#get>.
    pub async fn list_host_metric_values(
        &self,
        host_id: HostId,
        name: String,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metrics", host_id),
            vec![
                ("name", vec![&name]),
                ("from", vec![&from.timestamp().to_string()]),
                ("to", vec![&to.timestamp().to_string()]),
            ],
            client::empty_body(),
            |res: ListMetricValuesResponse| res.metrics,
        )
        .await
    }

    /// Posts service metric values.
    ///
    /// See <https://mackerel.io/api-docs/entry/service-metrics#post>.
    pub async fn post_service_metric_values(
        &self,
        service_name: ServiceName,
        service_metric_values: Vec<ServiceMetricValue>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/services/{}/tsdb", service_name),
            vec![],
            Some(service_metric_values),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Fetches service metric values.
    ///
    /// See <https://mackerel.io/api-docs/entry/service-metrics#get>.
    pub async fn list_service_metric_values(
        &self,
        service_name: ServiceName,
        name: String,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metrics", service_name),
            vec![
                ("name", vec![&name]),
                ("from", vec![&from.timestamp().to_string()]),
                ("to", vec![&to.timestamp().to_string()]),
            ],
            client::empty_body(),
            |res: ListMetricValuesResponse| res.metrics,
        )
        .await
    }
}
