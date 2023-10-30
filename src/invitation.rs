use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

use crate::client;
use crate::error::Result;
use crate::response;
use crate::user::UserAuthority;

/// An invitation
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

impl client::Client {
    /// Fetches all the invitations.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#list>.
    pub async fn list_invitations(&self) -> Result<Vec<Invitation>> {
        self.request(
            Method::GET,
            "/api/v0/invitations",
            vec![],
            client::empty_body(),
            response! { invitations: Vec<Invitation> },
        )
        .await
    }

    /// Creates a new invitation.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#create>.
    pub async fn create_invitation(&self, invitation_value: InvitationValue) -> Result<Invitation> {
        self.request(
            Method::POST,
            "/api/v0/invitations",
            vec![],
            Some(invitation_value),
            |invitation: Invitation| invitation,
        )
        .await
    }

    /// Revokes an invitation.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#revoke>.
    pub async fn revoke_invitation(&self, email: String) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/invitations/revoke",
            vec![],
            Some(HashMap::from([("email", email)])),
            |_: serde_json::Value| (),
        )
        .await
    }
}
