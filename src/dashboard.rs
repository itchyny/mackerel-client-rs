/// A dashboard
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dashboard {
    pub id: Option<String>,
    pub title: String,
    pub body_markdown: String,
    pub url_path: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use dashboard::*;

    fn dashboard_example() -> Dashboard {
        Dashboard {
            id: Some("abcde1".to_string()),
            title: "This is a dashboard".to_string(),
            body_markdown: "# Example\n[example](https://example.com)".to_string(),
            url_path: "example".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "id": "abcde1",
                "title": "This is a dashboard",
                "bodyMarkdown": "# Example\n[example](https://example.com)",
                "urlPath": "example"
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_dashboard() {
        assert_eq!(json_example(),
                   serde_json::to_value(&dashboard_example()).unwrap());
    }

    #[test]
    fn deserialize_dashboard() {
        assert_eq!(dashboard_example(),
                   serde_json::from_value(json_example()).unwrap());
    }

}
