use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

use crate::client;
use crate::error::Result;
use crate::name::Name;

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
    use super::*;
    use rstest::rstest;
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

    #[rstest]
    #[case(organization_example(), json_example())]
    fn test_organization_json(#[case] organization: Organization, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&organization).unwrap(), json);
        assert_eq!(organization, serde_json::from_value(json).unwrap());
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
