use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client::Client;
use crate::error::Result;
use crate::macros::*;
use crate::name::Name;

/// An organization value
#[derive(PartialEq, Eq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
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
        Organization::builder().name("ExampleOrganization").build()
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

impl Client {
    /// Retrieve the information on the organization.
    ///
    /// See <https://mackerel.io/api-docs/entry/organizations#get>.
    pub async fn get_organization(&self) -> Result<Organization> {
        self.request(
            Method::GET,
            "/api/v0/org",
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use crate::organization::*;
    use crate::tests::*;

    #[async_std::test]
    async fn get_organization() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/org",
            response = json!({ "name": "ExampleOrganization" }),
        };
        let organization = Organization {
            name: OrganizationName::from("ExampleOrganization"),
        };
        assert_eq!(
            test_client!(server).get_organization().await,
            Ok(organization)
        );
    }
}
