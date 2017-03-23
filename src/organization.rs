#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use organization::*;

    fn organization_example() -> Organization {
        Organization { name: "FooOrganization".to_string() }
    }

    fn json_example() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "name": "FooOrganization"
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_organization() {
        assert_eq!(json_example(),
                   serde_json::to_value(&organization_example()).unwrap());
    }

    #[test]
    fn deserialize_organization() {
        assert_eq!(organization_example(),
                   serde_json::from_value(json_example()).unwrap());
    }

}
