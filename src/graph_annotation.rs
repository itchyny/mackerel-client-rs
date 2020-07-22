use client;
use errors::*;
use reqwest::Method;

/// A graph annotation
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct GraphAnnotation {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub from: u64,
    pub to: u64,
    pub service: String,
    pub roles: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use graph_annotation::*;
    use serde_json;

    fn graph_annotation_example() -> GraphAnnotation {
        GraphAnnotation {
            id: Some("abcde1".to_string()),
            title: "Deploy application".to_string(),
            description: "Graph Annotation Example\nhttps://example.com".to_string(),
            from: 1484000000,
            to: 1484000030,
            service: "ExampleService".to_string(),
            roles: Some(vec!["ExampleRole1".to_string(), "ExampleRole2".to_string()]),
        }
    }

    fn json_example() -> serde_json::Value {
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

    #[test]
    fn serialize_graph_annotation() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&graph_annotation_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_graph_annotation() {
        assert_eq!(
            graph_annotation_example(),
            serde_json::from_value(json_example()).unwrap()
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
    /// See https://mackerel.io/api-docs/entry/graph-annotations#get.
    pub fn list_graph_annotations(
        &self,
        service: &str,
        from: u64,
        to: u64,
    ) -> Result<Vec<GraphAnnotation>> {
        self.request(
            Method::GET,
            "/api/v0/graph-annotations",
            vec![
                ("service", vec![service]),
                ("from", vec![&from.to_string()]),
                ("to", vec![&to.to_string()]),
            ],
            client::empty_body(),
            |res: ListGraphAnnotationsResponse| res.graph_annotations,
        )
    }

    /// Creates a new graph annotation.
    ///
    /// See https://mackerel.io/api-docs/entry/graph-annotations#create.
    pub fn create_graph_annotation(
        &self,
        graph_annotation: GraphAnnotation,
    ) -> Result<GraphAnnotation> {
        self.request(
            Method::POST,
            "/api/v0/graph-annotations",
            vec![],
            Some(graph_annotation),
            |graph_annotation| graph_annotation,
        )
    }

    /// Updates a graph annotation.
    ///
    /// See https://mackerel.io/api-docs/entry/graph-annotations#update.
    pub fn update_graph_annotation(
        &self,
        graph_annotation: GraphAnnotation,
    ) -> Result<GraphAnnotation> {
        let graph_annotation_id: String = graph_annotation
            .clone()
            .id
            .ok_or("specify the id to update a graph_annotation")?;
        self.request(
            Method::PUT,
            format!("/api/v0/graph-annotations/{}", graph_annotation_id),
            vec![],
            Some(graph_annotation),
            |graph_annotation| graph_annotation,
        )
    }

    /// Deletes a graph annotation.
    ///
    /// See https://mackerel.io/api-docs/entry/graph-annotations#delete.
    pub fn delete_graph_annotation(&self, graph_annotation_id: String) -> Result<GraphAnnotation> {
        self.request(
            Method::DELETE,
            format!("/api/v0/graph-annotations/{}", graph_annotation_id),
            vec![],
            client::empty_body(),
            |graph_annotation| graph_annotation,
        )
    }
}
