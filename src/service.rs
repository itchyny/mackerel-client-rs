#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub memo: String,
    pub roles: Vec<String>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use service::*;

    fn service_example() -> Service {
        Service {
            name: "FooService".to_string(),
            memo: "service memo".to_string(),
            roles: vec!["role0".to_string(), "role1".to_string(), "role2".to_string()],
        }
    }

    fn json_example() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "name": "FooService",
                "memo": "service memo",
                "roles": [
                    "role0",
                    "role1",
                    "role2"
                ]
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_service() {
        assert_eq!(json_example(),
                   serde_json::to_value(&service_example()).unwrap());
    }

    #[test]
    fn deserialize_service() {
        assert_eq!(service_example(),
                   serde_json::from_value(json_example()).unwrap());
    }

}
