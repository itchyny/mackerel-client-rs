use crate::client;
use crate::error::*;
use crate::name::Name;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// An organization
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub name: OrganizationName,
}

/// An organization name
/// ```rust
/// use mackerel_client::organization::OrganizationName;
///
/// let organization_name = OrganizationName::from("ExampleOrganization");
/// ```
pub type OrganizationName = Name<Organization>;

#[cfg(test)]
mod tests {
    use crate::organization::*;
    use serde_json::json;

    fn organization_example() -> Organization {
        Organization {
            name: "ExampleOrganization".into(),
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "ExampleOrganization"
        })
    }

    #[test]
    fn serialize_organization() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&organization_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_organization() {
        assert_eq!(
            organization_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }
}

impl client::Client {
    /// Retrieve the information on the organization.
    ///
    /// See <https://mackerel.io/api-docs/entry/organizations#get>.
    pub async fn get_organization(&self) -> Result<Organization> {
        self.request(
            Method::GET,
            "/api/v0/org",
            vec![],
            client::empty_body(),
            |org| org,
        )
        .await
    }
}
