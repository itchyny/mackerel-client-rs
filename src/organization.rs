use http::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::name::Name;

/// An organization value
#[derive(PartialEq, Eq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    pub name: OrganizationName,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
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

    fn organization_example1() -> Organization {
        Organization::builder().name("ExampleOrganization").build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "name": "ExampleOrganization"
        })
    }

    fn organization_example2() -> Organization {
        Organization::builder()
            .name("ExampleOrganization")
            .display_name("Example Organization")
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "name": "ExampleOrganization",
            "displayName": "Example Organization"
        })
    }

    #[rstest]
    #[case(organization_example1(), json_example1())]
    #[case(organization_example2(), json_example2())]
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
        assert_eq!(
            test_client!(server).get_organization().await,
            Ok(Organization::builder().name("ExampleOrganization").build())
        );
    }
}
