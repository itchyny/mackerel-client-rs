use crate::client;
use crate::entity::Id;
use crate::error::*;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

/// A user
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    pub is_in_registration_process: bool,
    #[serde(rename = "isMFAEnabled")]
    pub is_mfa_enabled: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authentication_methods: Vec<AuthenticationMethod>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub joined_at: DateTime<Utc>,
    #[serde(flatten)]
    pub value: UserValue,
}

impl std::ops::Deref for User {
    type Target = UserValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// A user id
pub type UserId = Id<UserValue>;

/// A user value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserValue {
    pub screen_name: String,
    pub email: String,
    pub authority: UserAuthority,
}

/// Authentication methods
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum AuthenticationMethod {
    Password,
    GitHub,
    IDCF,
    Google,
    Nifty,
    Yammer,
    KDDI,
}

/// User authority
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum UserAuthority {
    Owner,
    Manager,
    Collaborator,
    Viewer,
}

#[cfg(test)]
mod tests {
    use crate::user::*;
    use serde_json::json;

    fn user_example() -> User {
        User {
            id: "abcde".into(),
            is_in_registration_process: false,
            is_mfa_enabled: false,
            authentication_methods: vec![AuthenticationMethod::Password],
            joined_at: DateTime::from_timestamp(1630000000, 0).unwrap(),
            value: UserValue {
                screen_name: "Example Mackerel".to_string(),
                email: "mackerel@example.com".to_string(),
                authority: UserAuthority::Collaborator,
            },
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "id": "abcde",
            "screenName": "Example Mackerel",
            "email": "mackerel@example.com",
            "authority": "collaborator",
            "isInRegistrationProcess": false,
            "isMFAEnabled": false,
            "authenticationMethods": ["password"],
            "joinedAt": 1630000000,
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

    #[test]
    fn authentication_methods() {
        let test_cases = [
            (AuthenticationMethod::Password, "password"),
            (AuthenticationMethod::GitHub, "github"),
            (AuthenticationMethod::IDCF, "idcf"),
            (AuthenticationMethod::Google, "google"),
            (AuthenticationMethod::Nifty, "nifty"),
            (AuthenticationMethod::Yammer, "yammer"),
            (AuthenticationMethod::KDDI, "kddi"),
        ];
        for &(authentication_method, authentication_method_str) in &test_cases {
            assert_eq!(authentication_method.to_string(), authentication_method_str);
            assert_eq!(
                authentication_method,
                authentication_method_str.parse().unwrap()
            );
            assert_eq!(
                authentication_method,
                serde_json::from_value(authentication_method_str.into()).unwrap()
            );
            assert_eq!(
                serde_json::to_value(authentication_method).unwrap(),
                authentication_method_str
            );
        }
    }

    #[test]
    fn user_authorities() {
        let test_cases = [
            (UserAuthority::Owner, "owner"),
            (UserAuthority::Manager, "manager"),
            (UserAuthority::Collaborator, "collaborator"),
            (UserAuthority::Viewer, "viewer"),
        ];
        for &(user_authority, user_authority_str) in &test_cases {
            assert_eq!(user_authority.to_string(), user_authority_str);
            assert_eq!(user_authority, user_authority_str.parse().unwrap());
            assert_eq!(
                user_authority,
                serde_json::from_value(user_authority_str.into()).unwrap()
            );
            assert_eq!(
                serde_json::to_value(user_authority).unwrap(),
                user_authority_str
            );
        }
    }
}

#[derive(Deserialize)]
struct ListUsersResponse {
    users: Vec<User>,
}

impl client::Client {
    /// Fetches all the users.
    ///
    /// See <https://mackerel.io/api-docs/entry/users#list>.
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
    /// See <https://mackerel.io/api-docs/entry/users#delete>.
    pub async fn delete_user(&self, user_id: UserId) -> Result<User> {
        self.request(
            Method::DELETE,
            format!("/api/v0/users/{}", user_id),
            vec![],
            client::empty_body(),
            |user| user,
        )
        .await
    }
}
