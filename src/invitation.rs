use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::user::UserAuthority;

/// An invitation entity
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(flatten)]
    pub value: InvitationValue,
}

impl std::ops::Deref for Invitation {
    type Target = InvitationValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// An invitation value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct InvitationValue {
    pub email: String,
    pub authority: UserAuthority,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn invitation_example1() -> Invitation {
        Invitation::builder()
            .expires_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .value(
                InvitationValue::builder()
                    .email("example1@example.com")
                    .authority(UserAuthority::Manager)
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "expiresAt": 1700000000,
            "email": "example1@example.com",
            "authority": "manager"
        })
    }

    fn invitation_example2() -> Invitation {
        Invitation::builder()
            .expires_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .value(
                InvitationValue::builder()
                    .email("example2@example.com")
                    .authority(UserAuthority::Collaborator)
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "expiresAt": 1700000000,
            "email": "example2@example.com",
            "authority": "collaborator"
        })
    }

    #[rstest]
    #[case(invitation_example1(), json_example1())]
    #[case(invitation_example2(), json_example2())]
    fn test_invitation_json(#[case] invitation: Invitation, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&invitation).unwrap(), json);
        assert_eq!(invitation, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Fetches all the invitations.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#list>.
    pub async fn list_invitations(&self) -> Result<Vec<Invitation>> {
        self.request(
            Method::GET,
            "/api/v0/invitations",
            query_params![],
            request_body![],
            response_body! { invitations: Vec<Invitation> },
        )
        .await
    }

    /// Creates a new invitation.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#create>.
    pub async fn create_invitation(
        &self,
        invitation_value: impl Borrow<InvitationValue>,
    ) -> Result<Invitation> {
        self.request(
            Method::POST,
            "/api/v0/invitations",
            query_params![],
            request_body!(invitation_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Revokes an invitation.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#revoke>.
    pub async fn revoke_invitation(&self, email: impl AsRef<str>) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/invitations/revoke",
            query_params![],
            request_body! { email: String = email.as_ref().to_owned() },
            response_body!(),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use chrono::DateTime;
    use serde_json::json;

    use crate::invitation::*;
    use crate::tests::*;

    fn value_example() -> InvitationValue {
        InvitationValue::builder()
            .email("mackerel@example.com")
            .authority(UserAuthority::Manager)
            .build()
    }

    fn entity_example() -> Invitation {
        Invitation::builder()
            .expires_at(DateTime::from_timestamp(1698937200, 0).unwrap())
            .value(value_example())
            .build()
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "email": "mackerel@example.com",
            "authority": "manager",
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["expiresAt"] = json!(1698937200);
        json
    }

    #[async_std::test]
    async fn list_invitations() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/invitations",
            response = json!({
                "invitations": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_invitations().await,
            Ok(vec![entity_example()])
        );
    }

    #[async_std::test]
    async fn create_invitation() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/invitations",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .create_invitation(value_example())
                .await,
            Ok(entity_example())
        );
        assert_eq!(
            test_client!(server)
                .create_invitation(&value_example())
                .await,
            Ok(entity_example())
        );
    }

    #[async_std::test]
    async fn revoke_invitation() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/invitations/revoke",
            request = json!({ "email": "mackerel@example.com" }),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .revoke_invitation("mackerel@example.com")
                .await,
            Ok(()),
        );
        assert_eq!(
            test_client!(server)
                .revoke_invitation(String::from("mackerel@example.com"))
                .await,
            Ok(()),
        );
    }
}
