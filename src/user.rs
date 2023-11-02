use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::Client;
use crate::entity::Id;
use crate::error::Result;
use crate::macros::*;

/// A user entity
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: UserId,
    #[builder(default)]
    pub is_in_registration_process: bool,
    #[builder(default)]
    #[serde(rename = "isMFAEnabled")]
    pub is_mfa_enabled: bool,
    #[builder(default)]
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
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct UserValue {
    pub screen_name: String,
    pub email: String,
    pub authority: UserAuthority,
}

/// Authentication method
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
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn user_example1() -> User {
        User::builder()
            .id("abcde1")
            .joined_at(DateTime::from_timestamp(1630000000, 0).unwrap())
            .value(
                UserValue::builder()
                    .screen_name("Example User 1")
                    .email("mackerel@example.com")
                    .authority(UserAuthority::Manager)
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "screenName": "Example User 1",
            "email": "mackerel@example.com",
            "authority": "manager",
            "isInRegistrationProcess": false,
            "isMFAEnabled": false,
            "joinedAt": 1630000000,
        })
    }

    fn user_example2() -> User {
        User::builder()
            .id("abcde2")
            .is_in_registration_process(true)
            .is_mfa_enabled(true)
            .authentication_methods([AuthenticationMethod::Password, AuthenticationMethod::GitHub])
            .joined_at(DateTime::from_timestamp(1630000000, 0).unwrap())
            .value(
                UserValue::builder()
                    .screen_name("Example User 2")
                    .email("mackerel@example.com")
                    .authority(UserAuthority::Collaborator)
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "screenName": "Example User 2",
            "email": "mackerel@example.com",
            "authority": "collaborator",
            "isInRegistrationProcess": true,
            "isMFAEnabled": true,
            "authenticationMethods": ["password", "github"],
            "joinedAt": 1630000000,
        })
    }

    #[rstest]
    #[case(user_example1(), json_example1())]
    #[case(user_example2(), json_example2())]
    fn test_user_json(#[case] user: User, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&user).unwrap(), json);
        assert_eq!(user, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(AuthenticationMethod::Password, "password")]
    #[case(AuthenticationMethod::GitHub, "github")]
    #[case(AuthenticationMethod::IDCF, "idcf")]
    #[case(AuthenticationMethod::Google, "google")]
    #[case(AuthenticationMethod::Nifty, "nifty")]
    #[case(AuthenticationMethod::Yammer, "yammer")]
    #[case(AuthenticationMethod::KDDI, "kddi")]
    fn test_authentication_method(
        #[case] authentication_method: AuthenticationMethod,
        #[case] authentication_method_str: &str,
    ) {
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

    #[rstest]
    #[case(UserAuthority::Owner, "owner")]
    #[case(UserAuthority::Manager, "manager")]
    #[case(UserAuthority::Collaborator, "collaborator")]
    #[case(UserAuthority::Viewer, "viewer")]
    fn test_user_authority(
        #[case] user_authority: UserAuthority,
        #[case] user_authority_str: &str,
    ) {
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

impl Client {
    /// Fetches all the users.
    ///
    /// See <https://mackerel.io/api-docs/entry/users#list>.
    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.request(
            Method::GET,
            "/api/v0/users",
            query_params![],
            request_body![],
            response_body! { users: Vec<User> },
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
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use chrono::DateTime;
    use serde_json::json;

    use crate::tests::*;
    use crate::user::*;

    fn value_example() -> UserValue {
        UserValue::builder()
            .screen_name("Example User")
            .email("mackerel@example.com")
            .authority(UserAuthority::Manager)
            .build()
    }

    fn entity_example() -> User {
        User::builder()
            .id("user0")
            .joined_at(DateTime::from_timestamp(1699200000, 0).unwrap())
            .value(value_example())
            .build()
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "screenName": "Example User",
            "email": "mackerel@example.com",
            "authority": "manager",
            "isInRegistrationProcess": false,
            "isMFAEnabled": false,
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("user0");
        json["joinedAt"] = json!(1699200000);
        json
    }

    #[async_std::test]
    async fn list_users() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/users",
            response = json!({
                "users": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_users().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn delete_user() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/users/user0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).delete_user("user0".into()).await,
            Ok(entity_example()),
        );
    }
}
