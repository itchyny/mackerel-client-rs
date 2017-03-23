#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use role::*;

    fn role_example() -> Role {
        Role {
            name: "FooRole".to_string(),
            memo: "role memo".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "name": "FooRole",
                "memo": "role memo"
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_role() {
        assert_eq!(json_example(),
                   serde_json::to_value(&role_example()).unwrap());
    }

    #[test]
    fn deserialize_role() {
        assert_eq!(role_example(),
                   serde_json::from_value(json_example()).unwrap());
    }

}
