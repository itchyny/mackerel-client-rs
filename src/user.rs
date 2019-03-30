use http::Method;
use client;
use errors::*;

use authority::Authority;

/// A user
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub screen_name: String,
    pub email: String,
    pub authority: Authority,
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
            authority: Authority::Collaborator,
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "id": "abcde",
            "screenName": "Example Mackerel",
            "email": "mackerel@example.com",
            "authority": "collaborator"
        })
    }

    #[test]
    fn serialize_user() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&user_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_user() {
        assert_eq!(
            user_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }

}

#[derive(Deserialize)]
struct ListUsersResponse {
    users: Vec<User>,
}

impl client::Client {
    /// Fetches all the services.
    ///
    /// See https://mackerel.io/api-docs/entry/users#list.
    pub fn list_users(&self) -> Result<Vec<User>> {
        self.request(
            Method::GET,
            "/api/v0/users",
            vec![],
            client::empty_body(),
            |res: ListUsersResponse| res.users,
        )
    }

    /// Delete the user from the organization.
    ///
    /// See https://mackerel.io/api-docs/entry/users#delete.
    pub fn delete_user(&self, user_name: &str) -> Result<User> {
        self.request(
            Method::DELETE,
            format!("/api/v0/users/{}", user_name),
            vec![],
            client::empty_body(),
            |user| user,
        )
    }
}
