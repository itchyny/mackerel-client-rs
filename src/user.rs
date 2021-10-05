use crate::authority::Authority;
use crate::client;
use crate::entity::{Entity, Id};
use crate::errors::*;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// A user
pub type User = Entity<UserValue>;

/// A user id
pub type UserId = Id<UserValue>;

/// A user value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserValue {
    pub screen_name: String,
    pub email: String,
    pub authority: Authority,
}

#[cfg(test)]
mod tests {
    use crate::user::*;
    use serde_json::json;

    fn user_example() -> User {
        User {
            id: "abcde".into(),
            value: UserValue {
                screen_name: "Example Mackerel".to_string(),
                email: "mackerel@example.com".to_string(),
                authority: Authority::Collaborator,
            },
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
    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.request(
            Method::GET,
            "/api/v0/users",
            vec![],
            client::empty_body(),
            |res: ListUsersResponse| res.users,
        )
        .await
    }

    /// Delete the user from the organization.
    ///
    /// See https://mackerel.io/api-docs/entry/users#delete.
    pub async fn delete_user(&self, user_name: &str) -> Result<User> {
        self.request(
            Method::DELETE,
            format!("/api/v0/users/{}", user_name),
            vec![],
            client::empty_body(),
            |user| user,
        )
        .await
    }
}
