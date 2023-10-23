use crate::client;
use crate::error::*;
use crate::host::HostId;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// A host metric value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostMetricValue {
    pub host_id: HostId,
    pub name: String,
    #[serde(flatten)]
    pub value: MetricValue,
}

/// A service metric value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMetricValue {
    pub name: String,
    #[serde(flatten)]
    pub value: MetricValue,
}

/// A metric value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    pub time: u64,
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use crate::metric::*;
    use serde_json::json;

    fn host_metric_example1() -> HostMetricValue {
        HostMetricValue {
            host_id: "abcde1".into(),
            name: "loadavg.loadavg1".to_string(),
            value: MetricValue {
                time: 1700000000,
                value: 1.2,
            },
        }
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
        ServiceMetricValue {
            name: "custom.metric.name".to_string(),
            value: MetricValue {
                time: 1700000000,
                value: 1.3,
            },
        }
    }

    fn service_metric_json_example1() -> serde_json::Value {
        json!({
            "name": "custom.metric.name",
            "time": 1700000000,
            "value": 1.3,
        })
    }

    #[test]
    fn serialize_metric_value() {
        assert_eq!(
            host_metric_json_example1(),
            serde_json::to_value(&host_metric_example1()).unwrap()
        );
        assert_eq!(
            service_metric_json_example1(),
            serde_json::to_value(&service_metric_example1()).unwrap()
        );
    }

    #[test]
    fn deserialize_metric_value() {
        assert_eq!(
            host_metric_example1(),
            serde_json::from_value(host_metric_json_example1()).unwrap()
        );
        assert_eq!(
            service_metric_example1(),
            serde_json::from_value(service_metric_json_example1()).unwrap()
        );
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
        from: u64,
        to: u64,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metrics", host_id),
            vec![
                ("name", vec![&name]),
                ("from", vec![&from.to_string()]),
                ("to", vec![&to.to_string()]),
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
        from: u64,
        to: u64,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metrics", service_name),
            vec![
                ("name", vec![&name]),
                ("from", vec![&from.to_string()]),
                ("to", vec![&to.to_string()]),
            ],
            client::empty_body(),
            |res: ListMetricValuesResponse| res.metrics,
        )
        .await
    }
}
