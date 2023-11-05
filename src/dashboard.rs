use chrono::{DateTime, Duration, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, DurationSeconds};
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
    #[builder(default)]
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
        #[serde(rename = "range", default, skip_serializing_if = "Option::is_none")]
        time_range: Option<DashboardTimeRange>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value_range: Option<DashboardValueRange>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        reference_lines: Vec<DashboardReferenceLine>,
        layout: DashboardLayout,
    },
    #[serde(rename_all = "camelCase")]
    Value {
        title: String,
        metric: DashboardMetric,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fraction_size: Option<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        suffix: Option<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_new_version: bool,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        shows_trend: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        format_rules: Vec<DashboardFormatRule>,
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
        role_fullname: RoleFullname,
        layout: DashboardLayout,
    },
}

/// A dashboard graph
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DashboardGraph {
    #[serde(rename_all = "camelCase")]
    Host { host_id: HostId, name: String },
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
    #[serde(rename_all = "camelCase")]
    Expression { expression: String },
    #[serde(rename_all = "camelCase")]
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
                    .widgets([
                        DashboardWidget::Graph {
                            title: "Graph title".to_string(),
                            graph: DashboardGraph::Host {
                                host_id: "host1".into(),
                                name: "loadavg5".to_string(),
                            },
                            time_range: Some(DashboardTimeRange::Relative {
                                period: Duration::seconds(86400),
                                offset: Duration::seconds(-3600),
                            }),
                            value_range: Some(DashboardValueRange {
                                min: Some(0.0),
                                max: Some(20.0),
                            }),
                            reference_lines: vec![DashboardReferenceLine {
                                label: "critical".to_owned(),
                                value: 15.0,
                            }],
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
                                role_fullname: "service:role".into(),
                                name: "cpu.{user,iowait,system}".to_string(),
                                is_stacked: true,
                            },
                            time_range: Some(DashboardTimeRange::Absolute {
                                start: DateTime::from_timestamp(1630000000, 0).unwrap(),
                                end: DateTime::from_timestamp(1630003600, 0).unwrap(),
                            }),
                            value_range: None,
                            reference_lines: vec![],
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
                                host_id: "host2".into(),
                                name: "cpu.user.percentage".to_string(),
                            },
                            fraction_size: Some(4),
                            suffix: Some("%".to_string()),
                            is_new_version: true,
                            shows_trend: true,
                            format_rules: vec![DashboardFormatRule {
                                name: "heavy usage".to_owned(),
                                threshold: 80.0,
                                operator: MonitorOperator::GreaterThan,
                            }],
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
                            role_fullname: "service:role".into(),
                            layout: DashboardLayout {
                                x: 8,
                                y: 6,
                                width: 8,
                                height: 6,
                            },
                        },
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
    pub async fn create_dashboard(&self, dashboard_value: &DashboardValue) -> Result<Dashboard> {
        self.request(
            Method::POST,
            "/api/v0/dashboards",
            query_params![],
            request_body!(dashboard_value),
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
        dashboard_value: &DashboardValue,
    ) -> Result<Dashboard> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/dashboards/{}", dashboard_id),
            query_params![],
            request_body!(dashboard_value),
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
            .widgets([DashboardWidget::Graph {
                title: "Graph title".to_string(),
                graph: DashboardGraph::Host {
                    host_id: "host0".into(),
                    name: "loadavg5".to_string(),
                },
                time_range: Some(DashboardTimeRange::Relative {
                    period: Duration::seconds(86400),
                    offset: Duration::seconds(-3600),
                }),
                value_range: Some(DashboardValueRange {
                    min: Some(0.0),
                    max: Some(20.0),
                }),
                reference_lines: vec![DashboardReferenceLine {
                    label: "critical".to_owned(),
                    value: 15.0,
                }],
                layout: DashboardLayout {
                    x: 0,
                    y: 0,
                    width: 8,
                    height: 6,
                },
            }])
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
                .update_dashboard("dashboard0", &value_example())
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
