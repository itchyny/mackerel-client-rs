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
    use serde_json;
    use graph_annotation::*;

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
        serde_json::from_str(r##"
            {
                "id": "abcde1",
                "title": "Deploy application",
                "description": "Graph Annotation Example\nhttps://example.com",
                "from": 1484000000,
                "to": 1484000030,
                "service": "ExampleService",
                "roles": ["ExampleRole1", "ExampleRole2"]
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_graph_annotation() {
        assert_eq!(json_example(),
                   serde_json::to_value(&graph_annotation_example()).unwrap());
    }

    #[test]
    fn deserialize_graph_annotation() {
        assert_eq!(graph_annotation_example(),
                   serde_json::from_value(json_example()).unwrap());
    }

}
