use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::host::HostId;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A dashboard
pub type Dashboard = Entity<DashboardValue>;

/// A dashboard id
pub type DashboardId = Id<DashboardValue>;

/// A dashboard value
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardValue {
    pub title: String,
    pub memo: String,
    pub url_path: String,
    #[serde(default)]
    pub widgets: Vec<DashboardWidget>,
}

/// A dashboard widget
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardWidget {
    #[serde(rename_all = "camelCase")]
    Graph {
        title: String,
        graph: DashboardGraph,
        range: Option<DashboardRange>,
        layout: DashboardLayout,
    },
    #[serde(rename_all = "camelCase")]
    Value {
        title: String,
        metric: DashboardMetric,
        fraction_size: Option<u64>,
        suffix: Option<String>,
        layout: DashboardLayout,
    },
    #[serde(rename_all = "camelCase")]
    Markdown {
        title: String,
        markdown: String,
        layout: DashboardLayout,
    },
    #[serde(rename_all = "camelCase")]
    AlertStatus {
        title: String,
        role_fullname: String,
        layout: DashboardLayout,
    },
}

/// A dashboard graph
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardGraph {
    #[serde(rename_all = "camelCase")]
    Host { host_id: HostId, name: String },
    #[serde(rename_all = "camelCase")]
    Role {
        role_fullname: String,
        name: String,
        is_stacked: Option<bool>,
    },
    #[serde(rename_all = "camelCase")]
    Service {
        service_name: ServiceName,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    Expression { expression: String },
    #[serde(rename_all = "camelCase")]
    Unknown {},
}

/// A dashboard metric
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardMetric {
    #[serde(rename_all = "camelCase")]
    Host { host_id: HostId, name: String },
    #[serde(rename_all = "camelCase")]
    Service {
        service_name: ServiceName,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    Expression { expression: String },
    #[serde(rename_all = "camelCase")]
    Unknown {},
}

/// A dashboard range
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardRange {
    #[serde(rename_all = "camelCase")]
    Relative { period: u64, offset: i64 },
    #[serde(rename_all = "camelCase")]
    Absolute { start: u64, end: u64 },
}

/// A dashboard layout
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}

#[cfg(test)]
mod tests {
    use crate::dashboard::*;
    use serde_json::json;

    fn dashboard_example() -> Dashboard {
        Dashboard {
            id: "abcde1".into(),
            value: DashboardValue {
                title: "This is a dashboard".to_string(),
                memo: "This is a dashboard memo.".to_string(),
                url_path: "example".to_string(),
                widgets: vec![
                    DashboardWidget::Graph {
                        title: "Graph title".to_string(),
                        graph: DashboardGraph::Host {
                            host_id: "abcde1".into(),
                            name: "loadavg5".to_string(),
                        },
                        range: Some(DashboardRange::Relative {
                            period: 86400,
                            offset: -3600,
                        }),
                        layout: DashboardLayout {
                            x: 0,
                            y: 0,
                            width: 8,
                            height: 6,
                        },
                    },
                    DashboardWidget::Graph {
                        title: "Graph title".to_string(),
                        graph: DashboardGraph::Role {
                            role_fullname: "service:role".to_string(),
                            name: "cpu.{user,iowait,system}".to_string(),
                            is_stacked: Some(true),
                        },
                        range: Some(DashboardRange::Absolute {
                            start: 1630000000,
                            end: 1630003600,
                        }),
                        layout: DashboardLayout {
                            x: 8,
                            y: 0,
                            width: 8,
                            height: 6,
                        },
                    },
                    DashboardWidget::Value {
                        title: "Metric value title".to_string(),
                        metric: DashboardMetric::Host {
                            host_id: "abcde1".into(),
                            name: "cpu.user.percentage".to_string(),
                        },
                        fraction_size: Some(4),
                        suffix: Some("%".to_string()),
                        layout: DashboardLayout {
                            x: 16,
                            y: 0,
                            width: 8,
                            height: 6,
                        },
                    },
                    DashboardWidget::Markdown {
                        title: "Markdown title".to_string(),
                        markdown: "# This is a markdown widget".to_string(),
                        layout: DashboardLayout {
                            x: 0,
                            y: 6,
                            width: 8,
                            height: 6,
                        },
                    },
                    DashboardWidget::AlertStatus {
                        title: "Alert status title".to_string(),
                        role_fullname: "service:role".to_string(),
                        layout: DashboardLayout {
                            x: 8,
                            y: 6,
                            width: 8,
                            height: 6,
                        },
                    },
                ],
            },
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "title": "This is a dashboard",
            "memo": "This is a dashboard memo.",
            "urlPath": "example",
            "widgets": [
                {
                    "type": "graph",
                    "title": "Graph title",
                    "graph": {
                        "type": "host",
                        "hostId": "abcde1",
                        "name": "loadavg5",
                    },
                    "range": {
                        "type": "relative",
                        "period": 86400,
                        "offset": -3600,
                    },
                    "layout": {
                        "x": 0,
                        "y": 0,
                        "width": 8,
                        "height": 6,
                    },
                },
                {
                    "type": "graph",
                    "title": "Graph title",
                    "graph": {
                        "type": "role",
                        "roleFullname": "service:role",
                        "name": "cpu.{user,iowait,system}",
                        "isStacked": true,
                    },
                    "range": {
                        "type": "absolute",
                        "start": 1630000000,
                        "end": 1630003600,
                    },
                    "layout": {
                        "x": 8,
                        "y": 0,
                        "width": 8,
                        "height": 6,
                    },
                },
                {
                    "type": "value",
                    "title": "Metric value title",
                    "metric": {
                        "type": "host",
                        "hostId": "abcde1",
                        "name": "cpu.user.percentage",
                    },
                    "fractionSize": 4,
                    "suffix": "%",
                    "layout": {
                        "x": 16,
                        "y": 0,
                        "width": 8,
                        "height": 6,
                    },
                },
                {
                    "type": "markdown",
                    "title": "Markdown title",
                    "markdown": "# This is a markdown widget",
                    "layout": {
                        "x": 0,
                        "y": 6,
                        "width": 8,
                        "height": 6,
                    },
                },
                {
                    "type": "alertStatus",
                    "title": "Alert status title",
                    "roleFullname": "service:role",
                    "layout": {
                        "x": 8,
                        "y": 6,
                        "width": 8,
                        "height": 6,
                    },
                },
            ],
        })
    }

    #[test]
    fn serialize_dashboard() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&dashboard_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_dashboard() {
        assert_eq!(
            dashboard_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }
}

#[derive(Deserialize)]
struct ListDashboardsResponse {
    dashboards: Vec<Dashboard>,
}

impl client::Client {
    /// Fetches all the dashboards.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#list>.
    pub async fn list_dashboards(&self) -> Result<Vec<Dashboard>> {
        self.request(
            Method::GET,
            "/api/v0/dashboards",
            vec![],
            client::empty_body(),
            |res: ListDashboardsResponse| res.dashboards,
        )
        .await
    }

    /// Creates a new dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#create>.
    pub async fn create_dashboard(&self, dashboard_value: DashboardValue) -> Result<Dashboard> {
        self.request(
            Method::POST,
            "/api/v0/dashboards",
            vec![],
            Some(dashboard_value),
            |dashboard| dashboard,
        )
        .await
    }

    /// Gets a dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#get>.
    pub async fn get_dashboard(&self, dashboard_id: DashboardId) -> Result<Dashboard> {
        self.request(
            Method::GET,
            format!("/api/v0/dashboards/{}", dashboard_id),
            vec![],
            client::empty_body(),
            |dashboard| dashboard,
        )
        .await
    }

    /// Updates a dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#update>.
    pub async fn update_dashboard(
        &self,
        dashboard_id: DashboardId,
        dashboard_value: DashboardValue,
    ) -> Result<Dashboard> {
        self.request(
            Method::PUT,
            format!("/api/v0/dashboards/{}", dashboard_id),
            vec![],
            Some(dashboard_value),
            |dashboard| dashboard,
        )
        .await
    }

    /// Deletes a dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#delete>.
    pub async fn delete_dashboard(&self, dashboard_id: DashboardId) -> Result<Dashboard> {
        self.request(
            Method::DELETE,
            format!("/api/v0/dashboards/{}", dashboard_id),
            vec![],
            client::empty_body(),
            |dashboard| dashboard,
        )
        .await
    }
}
