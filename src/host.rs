use crate::client;
use crate::entity::Id;
use crate::errors::*;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::default::Default;

// TODO: A host id
pub type HostId = Id<()>;

/// A host value
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostValue {
    pub name: String,
    pub display_name: Option<String>,
    pub custom_identifier: Option<String>,
    pub meta: HashMap<String, Value>,
    pub interfaces: Option<Vec<HostInterface>>,
    pub role_fullnames: Option<Vec<String>>,
    pub checks: Option<Vec<HostCheck>>,
}

#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInterface {
    pub name: String,
    pub mac_address: Option<String>,
    pub ipv4_addresses: Option<Vec<String>>,
    pub ipv6_addresses: Option<Vec<String>>,
    pub ip_address: Option<String>,
    pub ipv6_address: Option<String>,
}

#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostCheck {
    pub name: String,
    pub memo: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::host::*;
    use serde_json::json;

    fn host_value_example() -> HostValue {
        HostValue {
            name: "test-host".to_string(),
            ..HostValue::default()
        }
    }

    fn host_value_json_example() -> serde_json::Value {
        json!({
            "name": "test-host",
            "meta": {}
        })
    }

    #[test]
    fn serialize_create_host_param() {
        assert_eq!(
            serde_json::to_value(&host_value_example()).unwrap(),
            host_value_json_example()
        );
    }

    #[test]
    fn deserialize_create_host_param() {
        assert_eq!(
            host_value_example(),
            serde_json::from_value(host_value_json_example()).unwrap()
        );
    }
}

#[derive(Deserialize)]
struct CreateHostResponse {
    id: HostId,
}

impl client::Client {
    /// Creates a new host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#create.
    pub async fn create_host(&self, host: HostValue) -> Result<HostId> {
        self.request(
            Method::POST,
            "/api/v0/hosts",
            vec![],
            Some(host),
            |res: CreateHostResponse| res.id,
        )
        .await
    }
}
