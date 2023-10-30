use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::response;
use crate::role::RoleName;
use crate::service::ServiceName;

/// A graph annotation
pub type GraphAnnotation = Entity<GraphAnnotationValue>;

/// A graph annotation id
pub type GraphAnnotationId = Id<GraphAnnotationValue>;

/// A graph annotation value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct GraphAnnotationValue {
    pub title: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub from: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub to: DateTime<Utc>,
    pub service: ServiceName,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleName>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn graph_annotation_example1() -> GraphAnnotation {
        GraphAnnotation::builder()
            .id("abcde1")
            .value(
                GraphAnnotationValue::builder()
                    .title("Deploy application")
                    .description("Graph Annotation Example\nhttps://example.com")
                    .from(DateTime::from_timestamp(1484000000, 0).unwrap())
                    .to(DateTime::from_timestamp(1484000030, 0).unwrap())
                    .service("ExampleService")
                    .roles(["ExampleRole1".into(), "ExampleRole2".into()])
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "title": "Deploy application",
            "description": "Graph Annotation Example\nhttps://example.com",
            "from": 1484000000,
            "to": 1484000030,
            "service": "ExampleService",
            "roles": ["ExampleRole1", "ExampleRole2"]
        })
    }

    fn graph_annotation_example2() -> GraphAnnotation {
        GraphAnnotation::builder()
            .id("abcde2")
            .value(
                GraphAnnotationValue::builder()
                    .title("Deploy application")
                    .from(DateTime::from_timestamp(1484000000, 0).unwrap())
                    .to(DateTime::from_timestamp(1484000030, 0).unwrap())
                    .service("ExampleService")
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "title": "Deploy application",
            "from": 1484000000,
            "to": 1484000030,
            "service": "ExampleService",
        })
    }

    #[rstest]
    #[case(graph_annotation_example1(), json_example1())]
    #[case(graph_annotation_example2(), json_example2())]
    fn test_graph_annotation(
        #[case] graph_annotation: GraphAnnotation,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&graph_annotation).unwrap(), json);
        assert_eq!(graph_annotation, serde_json::from_value(json).unwrap());
    }
}

impl client::Client {
    /// Fetches graph annotations.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#get>.
    pub async fn list_graph_annotations(
        &self,
        service: ServiceName,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<GraphAnnotation>> {
        self.request(
            Method::GET,
            "/api/v0/graph-annotations",
            vec![
                ("service", vec![&service]),
                ("from", vec![&from.timestamp().to_string()]),
                ("to", vec![&to.timestamp().to_string()]),
            ],
            client::empty_body(),
            response! { graphAnnotations: Vec<GraphAnnotation> },
        )
        .await
    }

    /// Creates a new graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#create>.
    pub async fn create_graph_annotation(
        &self,
        graph_annotation_value: GraphAnnotationValue,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::POST,
            "/api/v0/graph-annotations",
            vec![],
            Some(graph_annotation_value),
            |graph_annotation| graph_annotation,
        )
        .await
    }

    /// Updates a graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#update>.
    pub async fn update_graph_annotation(
        &self,
        graph_annontation_id: GraphAnnotationId,
        graph_annotation_value: GraphAnnotationValue,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::PUT,
            format!("/api/v0/graph-annotations/{}", graph_annontation_id),
            vec![],
            Some(graph_annotation_value),
            |graph_annotation| graph_annotation,
        )
        .await
    }

    /// Deletes a graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#delete>.
    pub async fn delete_graph_annotation(
        &self,
        graph_annotation_id: GraphAnnotationId,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::DELETE,
            format!("/api/v0/graph-annotations/{}", graph_annotation_id),
            vec![],
            client::empty_body(),
            |graph_annotation| graph_annotation,
        )
        .await
    }
}
