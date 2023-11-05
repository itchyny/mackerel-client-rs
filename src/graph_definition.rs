use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;

/// A graph definition
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinition {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub display_name: String,
    #[builder(default)]
    #[serde(default)]
    pub unit: GraphUnit,
    #[builder(default)]
    #[serde(default)]
    pub metrics: Vec<GraphMetric>,
}

/// A graph metric
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct GraphMetric {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_stacked: bool,
}

/// A graph metric unit
#[derive(
    PartialEq, Clone, Debug, Default, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum GraphUnit {
    #[default]
    Float,
    Integer,
    Percentage,
    Seconds,
    Milliseconds,
    Bytes,
    #[strum(serialize = "bytes/sec")]
    BytesPerSec,
    #[strum(serialize = "bits/sec")]
    BitsPerSec,
    IOPS,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn graph_definition_example1() -> GraphDefinition {
        GraphDefinition::builder().name("custom.metric").build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "name": "custom.metric",
            "unit": "float",
            "metrics": [],
        })
    }

    fn graph_definition_example2() -> GraphDefinition {
        GraphDefinition::builder()
            .name("custom.metric")
            .display_name("This is a graph display name.")
            .unit(GraphUnit::Percentage)
            .metrics([
                GraphMetric::builder().name("custom.metric.foo").build(),
                GraphMetric::builder()
                    .name("custom.metric.bar")
                    .display_name("Metric bar")
                    .is_stacked(false)
                    .build(),
                GraphMetric::builder()
                    .name("custom.metric.baz")
                    .is_stacked(true)
                    .build(),
            ])
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "name": "custom.metric",
            "displayName": "This is a graph display name.",
            "unit": "percentage",
            "metrics": [
                { "name": "custom.metric.foo" },
                { "name": "custom.metric.bar", "displayName": "Metric bar" },
                { "name": "custom.metric.baz", "isStacked": true },
            ],
        })
    }

    #[rstest]
    #[case(graph_definition_example1(), json_example1())]
    #[case(graph_definition_example2(), json_example2())]
    fn test_graph_definition(
        #[case] graph_definition: GraphDefinition,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&graph_definition).unwrap(), json);
        assert_eq!(graph_definition, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(GraphUnit::Float, "float")]
    #[case(GraphUnit::Integer, "integer")]
    #[case(GraphUnit::Percentage, "percentage")]
    #[case(GraphUnit::Seconds, "seconds")]
    #[case(GraphUnit::Milliseconds, "milliseconds")]
    #[case(GraphUnit::Bytes, "bytes")]
    #[case(GraphUnit::BytesPerSec, "bytes/sec")]
    #[case(GraphUnit::BitsPerSec, "bits/sec")]
    #[case(GraphUnit::IOPS, "iops")]
    fn test_graph_unit(#[case] graph_unit: GraphUnit, #[case] graph_unit_str: &str) {
        assert_eq!(graph_unit.to_string(), graph_unit_str);
        assert_eq!(graph_unit, graph_unit_str.parse().unwrap());
        assert_eq!(
            graph_unit,
            serde_json::from_value(graph_unit_str.into()).unwrap()
        );
        assert_eq!(serde_json::to_value(graph_unit).unwrap(), graph_unit_str);
    }
}

impl Client {
    /// Creates new graph definitions.
    ///
    /// See <https://mackerel.io/api-docs/entry/host-metrics#post-graphdef>.
    pub async fn create_graph_definitions(
        &self,
        graph_definitions: impl IntoIterator<Item = GraphDefinition>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/graph-defs/create",
            query_params![],
            request_body!(graph_definitions.into_iter().collect::<Vec<_>>()),
            response_body!(),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use crate::graph_definition::*;
    use crate::tests::*;

    #[async_std::test]
    async fn create_graph_definitions() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/graph-defs/create",
            request = json!([
                {
                    "name": "custom.metric",
                    "displayName": "This is a graph display name.",
                    "unit": "percentage",
                    "metrics": [
                        { "name": "custom.metric.foo" },
                        { "name": "custom.metric.bar", "displayName": "Metric bar" },
                        { "name": "custom.metric.baz", "isStacked": true },
                    ],
                },
            ]),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .create_graph_definitions([GraphDefinition::builder()
                    .name("custom.metric")
                    .display_name("This is a graph display name.")
                    .unit(GraphUnit::Percentage)
                    .metrics([
                        GraphMetric::builder().name("custom.metric.foo").build(),
                        GraphMetric::builder()
                            .name("custom.metric.bar")
                            .display_name("Metric bar")
                            .is_stacked(false)
                            .build(),
                        GraphMetric::builder()
                            .name("custom.metric.baz")
                            .is_stacked(true)
                            .build(),
                    ])
                    .build()])
                .await,
            Ok(()),
        );
    }
}
