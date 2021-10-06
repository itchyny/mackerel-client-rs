use crate::client;
use crate::error::*;
use crate::host::HostId;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// A metric value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct MetricValue {
    pub time: u64,
    pub value: f64,
}

#[derive(Deserialize)]
struct ListMetricValuesResponse {
    metrics: Vec<MetricValue>,
}

impl client::Client {
    /// Fetches host metric value.
    ///
    /// See https://mackerel.io/api-docs/entry/host-metrics#get.
    pub async fn list_host_metric_values(
        &self,
        id: HostId,
        name: String,
        from: u64,
        to: u64,
    ) -> Result<Vec<MetricValue>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metrics", id),
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

    /// Fetches service metric values.
    ///
    /// See https://mackerel.io/api-docs/entry/service-metrics#get.
    pub async fn list_service_metric_values(
        &self,
        service_name: String,
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
