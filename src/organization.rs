use reqwest::Method::*;
use client;
use errors::*;

/// An organization
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Organization {
    pub name: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use organization::*;

    fn organization_example() -> Organization {
        Organization {
            name: "FooOrganization".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "FooOrganization"
        })
    }

    #[test]
    fn serialize_organization() {
        assert_eq!(json_example(), serde_json::to_value(&organization_example()).unwrap());
    }

    #[test]
    fn deserialize_organization() {
        assert_eq!(organization_example(), serde_json::from_value(json_example()).unwrap());
    }
}

impl client::Client {
    /// Retrieve the information on the organization.
    ///
    /// See https://mackerel.io/api-docs/entry/organizations#get.
    pub fn get_organization(&self) -> Result<Organization> {
        self.request(Get, "/api/v0/org", vec![], client::empty_body(), |org| org)
    }
}
