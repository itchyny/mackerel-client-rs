use crate::authority::Authority;
use crate::client;
use crate::error::*;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::FromIterator;

/// An invitation
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub expires_at: u64,
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
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitationValue {
    pub email: String,
    pub authority: Authority,
}

#[cfg(test)]
mod tests {
    use crate::invitation::*;
    use serde_json::json;

    fn invitation_example1() -> Invitation {
        Invitation {
            expires_at: 1700000000,
            value: InvitationValue {
                email: "example1@example.com".to_string(),
                authority: Authority::Manager,
            },
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "expiresAt": 1700000000,
            "email": "example1@example.com",
            "authority": "manager"
        })
    }

    fn invitation_example2() -> Invitation {
        Invitation {
            expires_at: 1700000000,
            value: InvitationValue {
                email: "example2@example.com".to_string(),
                authority: Authority::Collaborator,
            },
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "expiresAt": 1700000000,
            "email": "example2@example.com",
            "authority": "collaborator"
        })
    }

    fn invitation_example3() -> InvitationValue {
        InvitationValue {
            email: "example3@example.com".to_string(),
            authority: Authority::Viewer,
        }
    }

    fn json_example3() -> serde_json::Value {
        json!({
            "email": "example3@example.com",
            "authority": "viewer"
        })
    }

    #[test]
    fn serialize_invitation() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&invitation_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&invitation_example2()).unwrap()
        );
        assert_eq!(
            json_example3(),
            serde_json::to_value(&invitation_example3()).unwrap()
        );
    }

    #[test]
    fn deserialize_invitation() {
        assert_eq!(
            invitation_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            invitation_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
        assert_eq!(
            invitation_example3(),
            serde_json::from_value(json_example3()).unwrap()
        );
    }
}

#[derive(Deserialize)]
struct ListInvitationsResponse {
    invitations: Vec<Invitation>,
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
            |res: ListInvitationsResponse| res.invitations,
        )
        .await
    }

    /// Creates a new invitation.
    ///
    /// See <https://mackerel.io/api-docs/entry/invitations#create>.
    pub async fn create_invitation(&self, invitation: InvitationValue) -> Result<Invitation> {
        self.request(
            Method::POST,
            "/api/v0/invitations",
            vec![],
            Some(invitation),
            |res: Invitation| res,
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
            Some(HashMap::<_, _>::from_iter([("email", email)])),
            |_: serde_json::Value| (),
        )
        .await
    }
}
