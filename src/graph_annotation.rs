use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::role::RoleName;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A graph annotation
pub type GraphAnnotation = Entity<GraphAnnotationValue>;

/// A graph annotation id
pub type GraphAnnotationId = Id<GraphAnnotationValue>;

/// A graph annotation value
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct GraphAnnotationValue {
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub from: u64,
    pub to: u64,
    pub service: ServiceName,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleName>,
}

#[cfg(test)]
mod tests {
    use crate::graph_annotation::*;
    use serde_json::json;

    fn graph_annotation_example1() -> GraphAnnotation {
        GraphAnnotation {
            id: "abcde1".into(),
            value: GraphAnnotationValue {
                title: "Deploy application".to_string(),
                description: "Graph Annotation Example\nhttps://example.com".to_string(),
                from: 1484000000,
                to: 1484000030,
                service: "ExampleService".into(),
                roles: vec!["ExampleRole1".into(), "ExampleRole2".into()],
            },
        }
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
        GraphAnnotation {
            id: "abcde2".into(),
            value: GraphAnnotationValue {
                title: "Deploy application".to_string(),
                description: "".to_string(),
                from: 1484000000,
                to: 1484000030,
                service: "ExampleService".into(),
                roles: vec![],
            },
        }
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

    #[test]
    fn serialize_graph_annotation() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&graph_annotation_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&graph_annotation_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_graph_annotation() {
        assert_eq!(
            graph_annotation_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            graph_annotation_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListGraphAnnotationsResponse {
    graph_annotations: Vec<GraphAnnotation>,
}

impl client::Client {
    /// Fetches graph annotations.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#get>.
    pub async fn list_graph_annotations(
        &self,
        service: ServiceName,
        from: u64,
        to: u64,
    ) -> Result<Vec<GraphAnnotation>> {
        self.request(
            Method::GET,
            "/api/v0/graph-annotations",
            vec![
                ("service", vec![&service]),
                ("from", vec![&from.to_string()]),
                ("to", vec![&to.to_string()]),
            ],
            client::empty_body(),
            |res: ListGraphAnnotationsResponse| res.graph_annotations,
        )
        .await
    }

    /// Creates a new graph annotation.
    ///
    /// See <https://mackerel.io/api-docs/entry/graph-annotations#create>.
    pub async fn create_graph_annotation(
        &self,
        graph_annotation: GraphAnnotationValue,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::POST,
            "/api/v0/graph-annotations",
            vec![],
            Some(graph_annotation),
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
        graph_annotation: GraphAnnotationValue,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::PUT,
            format!("/api/v0/graph-annotations/{}", graph_annontation_id),
            vec![],
            Some(graph_annotation),
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
