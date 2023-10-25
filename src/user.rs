use crate::client;
use crate::entity::Id;
use crate::error::*;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

/// A user
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
    use crate::user::*;
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

    #[test]
    fn serialize_user() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&user_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&user_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_user() {
        assert_eq!(
            user_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            user_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
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
