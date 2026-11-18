use chrono::{DateTime, Duration, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::DurationSeconds;
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::host::HostId;
use crate::monitor::MonitorOperator;
use crate::role::RoleFullname;
use crate::service::ServiceName;

/// A dashboard entity
pub type Dashboard = Entity<DashboardValue>;

/// A dashboard id
pub type DashboardId = Id<DashboardValue>;

/// A dashboard value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct DashboardValue {
    pub title: String,
    #[builder(default)]
    pub memo: String,
    pub url_path: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<DateTime<Utc>>,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<DateTime<Utc>>,
    #[builder(
        default,
        setter(transform = |widgets: impl IntoIterator<Item = impl Into<DashboardWidget>>| widgets
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default)]
    pub widgets: Vec<DashboardWidget>,
}

/// A dashboard widget
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardWidget {
    Graph(DashboardGraphWidget),
    Value(DashboardValueWidget),
    Markdown(DashboardMarkdownWidget),
    AlertStatus(DashboardAlertStatusWidget),
}

/// A dashboard graph widget
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DashboardGraphWidget {
    pub title: String,
    pub graph: DashboardGraph,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<DashboardTimeRange>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_range: Option<DashboardValueRange>,
    #[builder(
        default,
        setter(transform = |reference_lines: impl IntoIterator<Item = DashboardReferenceLine>| reference_lines
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_lines: Vec<DashboardReferenceLine>,
    #[builder(
        default,
        setter(transform = |legends: impl IntoIterator<Item = impl Into<String>>| legends
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legend_list: Vec<String>,
    pub layout: DashboardLayout,
}

/// A dashboard value widget
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DashboardValueWidget {
    pub title: String,
    pub metric: DashboardMetric,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction_size: Option<u64>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_new_version: bool,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub shows_trend: bool,
    #[builder(
        default,
        setter(transform = |format_rules: impl IntoIterator<Item = DashboardFormatRule>| format_rules
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub format_rules: Vec<DashboardFormatRule>,
    pub layout: DashboardLayout,
}

/// A dashboard markdown widget
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DashboardMarkdownWidget {
    pub title: String,
    pub markdown: String,
    pub layout: DashboardLayout,
}

/// A dashboard alert status widget
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DashboardAlertStatusWidget {
    pub title: String,
    pub role_fullname: RoleFullname,
    pub layout: DashboardLayout,
}

impl From<DashboardGraphWidget> for DashboardWidget {
    fn from(widget: DashboardGraphWidget) -> Self {
        Self::Graph(widget)
    }
}

impl From<DashboardValueWidget> for DashboardWidget {
    fn from(widget: DashboardValueWidget) -> Self {
        Self::Value(widget)
    }
}

impl From<DashboardMarkdownWidget> for DashboardWidget {
    fn from(widget: DashboardMarkdownWidget) -> Self {
        Self::Markdown(widget)
    }
}

impl From<DashboardAlertStatusWidget> for DashboardWidget {
    fn from(widget: DashboardAlertStatusWidget) -> Self {
        Self::AlertStatus(widget)
    }
}

/// A dashboard graph
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardGraph {
    #[serde(rename_all = "camelCase")]
    Host {
        host_id: HostId,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    Role {
        role_fullname: RoleFullname,
        name: String,
        is_stacked: bool,
    },
    #[serde(rename_all = "camelCase")]
    Service {
        service_name: ServiceName,
        name: String,
    },
    Expression {
        expression: String,
    },
    Query {
        query: String,
        legend: String,
    },
    Unknown {},
}

/// A dashboard metric
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardMetric {
    #[serde(rename_all = "camelCase")]
    Host {
        host_id: HostId,
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    Service {
        service_name: ServiceName,
        name: String,
    },
    Expression {
        expression: String,
    },
    Query {
        query: String,
        legend: String,
    },
    Unknown {},
}

/// A dashboard time range
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DashboardTimeRange {
    Relative {
        #[serde(with = "serde_with::As::<DurationSeconds<i64>>")]
        period: Duration,
        #[serde(with = "serde_with::As::<DurationSeconds<i64>>")]
        offset: Duration,
    },
    Absolute {
        #[serde(with = "chrono::serde::ts_seconds")]
        start: DateTime<Utc>,
        #[serde(with = "chrono::serde::ts_seconds")]
        end: DateTime<Utc>,
    },
}

/// A dashboard value range
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DashboardValueRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// A dashboard reference line
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DashboardReferenceLine {
    pub label: String,
    pub value: f64,
}

/// A dashboard format rule
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DashboardFormatRule {
    pub name: String,
    pub threshold: f64,
    pub operator: MonitorOperator,
}

/// A dashboard layout
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DashboardLayout {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn dashboard_example() -> Dashboard {
        Dashboard::builder()
            .id("dashboard1")
            .value(
                DashboardValue::builder()
                    .title("This is a dashboard")
                    .memo("This is a dashboard memo.")
                    .url_path("example")
                    .created_at(DateTime::from_timestamp(1630000000, 0).unwrap())
                    .updated_at(DateTime::from_timestamp(1630003600, 0).unwrap())
                    .widgets([
                        DashboardWidget::Graph(
                            DashboardGraphWidget::builder()
                                .title("Graph title")
                                .graph(DashboardGraph::Host {
                                    host_id: "host1".into(),
                                    name: "loadavg5".to_string(),
                                })
                                .time_range(DashboardTimeRange::Relative {
                                    period: Duration::seconds(86400),
                                    offset: Duration::seconds(-3600),
                                })
                                .value_range(DashboardValueRange {
                                    min: Some(0.0),
                                    max: Some(20.0),
                                })
                                .reference_lines([DashboardReferenceLine {
                                    label: "critical".to_owned(),
                                    value: 15.0,
                                }])
                                .legend_list(["loadavg.loadavg1", "loadavg.loadavg5"])
                                .layout(DashboardLayout {
                                    x: 0,
                                    y: 0,
                                    width: 8,
                                    height: 6,
                                })
                                .build(),
                        ),
                        DashboardWidget::Graph(
                            DashboardGraphWidget::builder()
                                .title("Graph title")
                                .graph(DashboardGraph::Role {
                                    role_fullname: "service:role".into(),
                                    name: "cpu.{user,iowait,system}".to_string(),
                                    is_stacked: true,
                                })
                                .time_range(DashboardTimeRange::Absolute {
                                    start: DateTime::from_timestamp(1630000000, 0).unwrap(),
                                    end: DateTime::from_timestamp(1630003600, 0).unwrap(),
                                })
                                .layout(DashboardLayout {
                                    x: 8,
                                    y: 0,
                                    width: 8,
                                    height: 6,
                                })
                                .build(),
                        ),
                        DashboardWidget::Value(
                            DashboardValueWidget::builder()
                                .title("Metric value title")
                                .metric(DashboardMetric::Host {
                                    host_id: "host2".into(),
                                    name: "cpu.user.percentage".to_string(),
                                })
                                .fraction_size(4)
                                .suffix("%")
                                .is_new_version(true)
                                .shows_trend(true)
                                .format_rules([DashboardFormatRule {
                                    name: "heavy usage".to_owned(),
                                    threshold: 80.0,
                                    operator: MonitorOperator::GreaterThan,
                                }])
                                .layout(DashboardLayout {
                                    x: 16,
                                    y: 0,
                                    width: 8,
                                    height: 6,
                                })
                                .build(),
                        ),
                        DashboardWidget::Markdown(
                            DashboardMarkdownWidget::builder()
                                .title("Markdown title")
                                .markdown("# This is a markdown widget")
                                .layout(DashboardLayout {
                                    x: 0,
                                    y: 6,
                                    width: 8,
                                    height: 6,
                                })
                                .build(),
                        ),
                        DashboardWidget::AlertStatus(
                            DashboardAlertStatusWidget::builder()
                                .title("Alert status title")
                                .role_fullname("service:role")
                                .layout(DashboardLayout {
                                    x: 8,
                                    y: 6,
                                    width: 8,
                                    height: 6,
                                })
                                .build(),
                        ),
                    ])
                    .build(),
            )
            .build()
    }

    fn json_example() -> serde_json::Value {
        json!({
            "id": "dashboard1",
            "title": "This is a dashboard",
            "memo": "This is a dashboard memo.",
            "urlPath": "example",
            "createdAt": 1630000000,
            "updatedAt": 1630003600,
            "widgets": [
                {
                    "type": "graph",
                    "title": "Graph title",
                    "graph": {
                        "type": "host",
                        "hostId": "host1",
                        "name": "loadavg5",
                    },
                    "range": {
                        "type": "relative",
                        "period": 86400,
                        "offset": -3600,
                    },
                    "valueRange": {
                        "min": 0.0,
                        "max": 20.0,
                    },
                    "referenceLines": [
                        {
                            "label": "critical",
                            "value": 15.0,
                        },
                    ],
                    "legendList": ["loadavg.loadavg1", "loadavg.loadavg5"],
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
                        "hostId": "host2",
                        "name": "cpu.user.percentage",
                    },
                    "fractionSize": 4,
                    "suffix": "%",
                    "isNewVersion": true,
                    "showsTrend": true,
                    "formatRules": [
                        {
                            "name": "heavy usage",
                            "threshold": 80.0,
                            "operator": ">",
                        },
                    ],
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

    #[rstest]
    #[case(dashboard_example(), json_example())]
    fn test_dashboard_json(#[case] dashboard: Dashboard, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&dashboard).unwrap(), json);
        assert_eq!(dashboard, serde_json::from_value(json).unwrap());
    }

    fn dashboard_graph_example1() -> DashboardGraph {
        DashboardGraph::Host {
            host_id: "host1".into(),
            name: "loadavg5".to_string(),
        }
    }

    fn json_dashboard_graph_example1() -> serde_json::Value {
        json!({
            "type": "host",
            "hostId": "host1",
            "name": "loadavg5",
        })
    }

    fn dashboard_graph_example2() -> DashboardGraph {
        DashboardGraph::Role {
            role_fullname: "service:role".into(),
            name: "cpu.{user,iowait,system}".to_string(),
            is_stacked: true,
        }
    }

    fn json_dashboard_graph_example2() -> serde_json::Value {
        json!({
            "type": "role",
            "roleFullname": "service:role",
            "name": "cpu.{user,iowait,system}",
            "isStacked": true,
        })
    }

    fn dashboard_graph_example3() -> DashboardGraph {
        DashboardGraph::Service {
            service_name: "service".into(),
            name: "cpu.{user,iowait,system}".to_string(),
        }
    }

    fn json_dashboard_graph_example3() -> serde_json::Value {
        json!({
            "type": "service",
            "serviceName": "service",
            "name": "cpu.{user,iowait,system}",
        })
    }

    fn dashboard_graph_example4() -> DashboardGraph {
        DashboardGraph::Expression {
            expression: "min(role(\"service:role\", \"custom.foo.bar\"))".to_string(),
        }
    }

    fn json_dashboard_graph_example4() -> serde_json::Value {
        json!({
            "type": "expression",
            "expression": "min(role(\"service:role\", \"custom.foo.bar\"))",
        })
    }

    fn dashboard_graph_example5() -> DashboardGraph {
        DashboardGraph::Query {
            query: "container.cpu.utilization{label=\"value\"}".to_string(),
            legend: "cpu.utilization {{k8s.node.name}}".to_string(),
        }
    }

    fn json_dashboard_graph_example5() -> serde_json::Value {
        json!({
            "type": "query",
            "query": "container.cpu.utilization{label=\"value\"}",
            "legend": "cpu.utilization {{k8s.node.name}}",
        })
    }

    #[rstest]
    #[case(dashboard_graph_example1(), json_dashboard_graph_example1())]
    #[case(dashboard_graph_example2(), json_dashboard_graph_example2())]
    #[case(dashboard_graph_example3(), json_dashboard_graph_example3())]
    #[case(dashboard_graph_example4(), json_dashboard_graph_example4())]
    #[case(dashboard_graph_example5(), json_dashboard_graph_example5())]
    fn test_dashboard_graph_json(#[case] graph: DashboardGraph, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&graph).unwrap(), json);
        assert_eq!(graph, serde_json::from_value(json).unwrap());
    }

    fn dashboard_metric_example1() -> DashboardMetric {
        DashboardMetric::Host {
            host_id: "host1".into(),
            name: "loadavg5".to_string(),
        }
    }

    fn json_dashboard_metric_example1() -> serde_json::Value {
        json!({
            "type": "host",
            "hostId": "host1",
            "name": "loadavg5",
        })
    }

    fn dashboard_metric_example2() -> DashboardMetric {
        DashboardMetric::Service {
            service_name: "service".into(),
            name: "cpu.{user,iowait,system}".to_string(),
        }
    }

    fn json_dashboard_metric_example2() -> serde_json::Value {
        json!({
            "type": "service",
            "serviceName": "service",
            "name": "cpu.{user,iowait,system}",
        })
    }

    fn dashboard_metric_example3() -> DashboardMetric {
        DashboardMetric::Expression {
            expression: "min(role(\"service:role\", \"custom.foo.bar\"))".to_string(),
        }
    }

    fn json_dashboard_metric_example3() -> serde_json::Value {
        json!({
            "type": "expression",
            "expression": "min(role(\"service:role\", \"custom.foo.bar\"))",
        })
    }

    fn dashboard_metric_example4() -> DashboardMetric {
        DashboardMetric::Query {
            query: "container.cpu.utilization{label=\"value\"}".to_string(),
            legend: "cpu.utilization {{k8s.node.name}}".to_string(),
        }
    }

    fn json_dashboard_metric_example4() -> serde_json::Value {
        json!({
            "type": "query",
            "query": "container.cpu.utilization{label=\"value\"}",
            "legend": "cpu.utilization {{k8s.node.name}}",
        })
    }

    #[rstest]
    #[case(dashboard_metric_example1(), json_dashboard_metric_example1())]
    #[case(dashboard_metric_example2(), json_dashboard_metric_example2())]
    #[case(dashboard_metric_example3(), json_dashboard_metric_example3())]
    #[case(dashboard_metric_example4(), json_dashboard_metric_example4())]
    fn test_dashboard_metric_json(
        #[case] metric: DashboardMetric,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&metric).unwrap(), json);
        assert_eq!(metric, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Fetches all the dashboards.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#list>.
    pub async fn list_dashboards(&self) -> Result<Vec<Dashboard>> {
        self.request(
            Method::GET,
            "/api/v0/dashboards",
            query_params![],
            request_body![],
            response_body! { dashboards: Vec<Dashboard> },
        )
        .await
    }

    /// Creates a new dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#create>.
    pub async fn create_dashboard(
        &self,
        dashboard_value: impl Borrow<DashboardValue>,
    ) -> Result<Dashboard> {
        self.request(
            Method::POST,
            "/api/v0/dashboards",
            query_params![],
            request_body!(dashboard_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Gets a dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#get>.
    pub async fn get_dashboard(&self, dashboard_id: impl Into<DashboardId>) -> Result<Dashboard> {
        self.request(
            Method::GET,
            format_url!("/api/v0/dashboards/{}", dashboard_id),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Updates a dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#update>.
    pub async fn update_dashboard(
        &self,
        dashboard_id: impl Into<DashboardId>,
        dashboard_value: impl Borrow<DashboardValue>,
    ) -> Result<Dashboard> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/dashboards/{}", dashboard_id),
            query_params![],
            request_body!(dashboard_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Deletes a dashboard.
    ///
    /// See <https://mackerel.io/api-docs/entry/dashboards#delete>.
    pub async fn delete_dashboard(
        &self,
        dashboard_id: impl Into<DashboardId>,
    ) -> Result<Dashboard> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/dashboards/{}", dashboard_id),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::dashboard::*;
    use crate::tests::*;

    fn value_example() -> DashboardValue {
        DashboardValue::builder()
            .title("Example dashboard")
            .memo("This is a dashboard memo.")
            .url_path("example")
            .widgets([DashboardGraphWidget::builder()
                .title("Graph title")
                .graph(DashboardGraph::Host {
                    host_id: "host0".into(),
                    name: "loadavg5".to_string(),
                })
                .time_range(DashboardTimeRange::Relative {
                    period: Duration::seconds(86400),
                    offset: Duration::seconds(-3600),
                })
                .value_range(DashboardValueRange {
                    min: Some(0.0),
                    max: Some(20.0),
                })
                .reference_lines([DashboardReferenceLine {
                    label: "critical".to_owned(),
                    value: 15.0,
                }])
                .layout(DashboardLayout {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 6,
                })
                .build()])
            .build()
    }

    fn entity_example() -> Dashboard {
        Dashboard {
            id: DashboardId::from("dashboard0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "title": "Example dashboard",
            "memo": "This is a dashboard memo.",
            "urlPath": "example",
            "widgets": [
                {
                    "type": "graph",
                    "title": "Graph title",
                    "graph": {
                        "type": "host",
                        "hostId": "host0",
                        "name": "loadavg5",
                    },
                    "range": {
                        "type": "relative",
                        "period": 86400,
                        "offset": -3600,
                    },
                    "valueRange": {
                        "min": 0.0,
                        "max": 20.0,
                    },
                    "referenceLines": [
                        {
                            "label": "critical",
                            "value": 15.0,
                        },
                    ],
                    "layout": {
                        "x": 0,
                        "y": 0,
                        "width": 8,
                        "height": 6,
                    },
                },
            ],
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("dashboard0");
        json
    }

    #[async_std::test]
    async fn list_dashboards() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/dashboards",
            response = json!({
                "dashboards": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_dashboards().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn create_dashboard() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/dashboards",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).create_dashboard(value_example()).await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .create_dashboard(&value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn get_dashboard() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/dashboards/dashboard0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).get_dashboard("dashboard0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .get_dashboard(DashboardId::from("dashboard0"))
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn update_dashboard() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/dashboards/dashboard0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_dashboard("dashboard0", value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_dashboard(DashboardId::from("dashboard0"), &value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn delete_dashboard() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/dashboards/dashboard0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).delete_dashboard("dashboard0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_dashboard(DashboardId::from("dashboard0"))
                .await,
            Ok(entity_example()),
        );
    }
}
