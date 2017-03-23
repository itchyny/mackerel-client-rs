#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    #[serde(rename = "screenName")]
    pub screen_name: String,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use user::*;

    fn user_example() -> User {
        User {
            id: "abcde".to_string(),
            screen_name: "Example Mackerel".to_string(),
            email: "mackerel@example.com".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "id": "abcde",
                "screenName": "Example Mackerel",
                "email": "mackerel@example.com"
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_user() {
        assert_eq!(json_example(),
                   serde_json::to_value(&user_example()).unwrap());
    }

    #[test]
    fn deserialize_user() {
        assert_eq!(user_example(),
                   serde_json::from_value(json_example()).unwrap());
    }

}
