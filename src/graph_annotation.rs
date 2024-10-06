use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::role::RoleName;
use crate::service::ServiceName;

/// A graph annotation entity
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
    #[builder(
        default,
        setter(transform = |role_names: impl IntoIterator<Item = impl Into<RoleName>>| role_names
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
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
            .id("annotation1")
            .value(
                GraphAnnotationValue::builder()
                    .title("Deploy application")
                    .description("Graph Annotation Example\nhttps://example.com")
                    .from(DateTime::from_timestamp(1484000000, 0).unwrap())
                    .to(DateTime::from_timestamp(1484000030, 0).unwrap())
                    .service("service0")
                    .roles(["role1", "role2"])
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "annotation1",
            "title": "Deploy application",
            "description": "Graph Annotation Example\nhttps://example.com",
            "from": 1484000000,
            "to": 1484000030,
            "service": "service0",
            "roles": ["role1", "role2"],
        })
    }

    fn graph_annotation_example2() -> GraphAnnotation {
        GraphAnnotation::builder()
            .id("annotation2")
            .value(
                GraphAnnotationValue::builder()
                    .title("Deploy application")
                    .from(DateTime::from_timestamp(1484000000, 0).unwrap())
                    .to(DateTime::from_timestamp(1484000030, 0).unwrap())
                    .service("service0")
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "annotation2",
            "title": "Deploy application",
            "from": 1484000000,
            "to": 1484000030,
            "service": "service0",
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

impl Client {
    /// Fetches graph annotations.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#get>.
    pub async fn list_graph_annotations(
        &self,
        service: impl Into<ServiceName>,
        from: impl Into<DateTime<Utc>>,
        to: impl Into<DateTime<Utc>>,
    ) -> Result<Vec<GraphAnnotation>> {
        self.request(
            Method::GET,
            "/api/v0/graph-annotations",
            query_params! {
                service = service.into().to_string(),
                from = from.into().timestamp().to_string(),
                to = to.into().timestamp().to_string(),
            },
            request_body![],
            response_body! { graphAnnotations: Vec<GraphAnnotation> },
        )
        .await
    }

    /// Creates a new graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#create>.
    pub async fn create_graph_annotation(
        &self,
        graph_annotation_value: impl Borrow<GraphAnnotationValue>,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::POST,
            "/api/v0/graph-annotations",
            query_params![],
            request_body!(graph_annotation_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Updates a graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#update>.
    pub async fn update_graph_annotation(
        &self,
        graph_annotation_id: impl Into<GraphAnnotationId>,
        graph_annotation_value: impl Borrow<GraphAnnotationValue>,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/graph-annotations/{}", graph_annotation_id),
            query_params![],
            request_body!(graph_annotation_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Deletes a graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#delete>.
    pub async fn delete_graph_annotation(
        &self,
        graph_annotation_id: impl Into<GraphAnnotationId>,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/graph-annotations/{}", graph_annotation_id),
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
    use std::time::{Duration, SystemTime};

    use crate::graph_annotation::*;
    use crate::tests::*;

    fn value_example() -> GraphAnnotationValue {
        GraphAnnotationValue::builder()
            .title("Example graph annotation")
            .description("This is a graph annotation description.")
            .from(DateTime::from_timestamp(1698890400, 0).unwrap())
            .to(DateTime::from_timestamp(1698894000, 0).unwrap())
            .service("service0")
            .roles(["role1", "role2"])
            .build()
    }

    fn entity_example() -> GraphAnnotation {
        GraphAnnotation {
            id: GraphAnnotationId::from("annotation0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "title": "Example graph annotation",
            "description": "This is a graph annotation description.",
            "from": 1698890400,
            "to": 1698894000,
            "service": "service0",
            "roles": ["role1", "role2"]
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("annotation0");
        json
    }

    #[async_std::test]
    async fn list_graph_annotations() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/graph-annotations",
            query_params = "service=service0&from=1698850800&to=1698937200",
            response = json!({
                "graphAnnotations": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_graph_annotations(
                    "service0",
                    SystemTime::UNIX_EPOCH + Duration::from_secs(1698850800),
                    SystemTime::UNIX_EPOCH + Duration::from_secs(1698937200),
                )
                .await,
            Ok(vec![entity_example()]),
        );
        assert_eq!(
            test_client!(server)
                .list_graph_annotations(
                    ServiceName::from("service0"),
                    DateTime::from_timestamp(1698850800, 0).unwrap(),
                    DateTime::from_timestamp(1698937200, 0).unwrap(),
                )
                .await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn create_graph_annotation() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/graph-annotations",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .create_graph_annotation(value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .create_graph_annotation(&value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn update_graph_annotation() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/graph-annotations/annotation0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_graph_annotation("annotation0", value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_graph_annotation(GraphAnnotationId::from("annotation0"), &value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn delete_graph_annotation() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/graph-annotations/annotation0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .delete_graph_annotation("annotation0")
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_graph_annotation(GraphAnnotationId::from("annotation0"))
                .await,
            Ok(entity_example()),
        );
    }
}
