use crate::{client, errors::*};
use reqwest::Method;
use serde_json::{map::Map, Value};
use std::default;

#[derive(Debug, Deserialize)]
pub struct HostId {
    id: String,
}

// TODO: Define its fields.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface();

// TODO: Type as struct
pub type Meta = Map<String, Value>;
pub type Checks = Map<String, Value>;

// Input type for POST /api/v0/hosts
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHostParam {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
    pub meta: Meta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<Interface>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_fullnames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<Checks>,
}

impl default::Default for CreateHostParam {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            display_name: None,
            custom_identifier: None,
            meta: Meta::default(),
            interfaces: None,
            role_fullnames: None,
            checks: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::host::*;

    fn json_example() -> serde_json::Value {
        json!({
            "name": "test-host",
            "meta": {}
        })
    }

    fn create_host_param_example() -> CreateHostParam {
        create_host_param! ({
            name -> "test-host".to_string()
        })
    }

    #[test]
    fn host_create_param_macro() {
        assert_eq!(create_host_param!({}), CreateHostParam::default());

        assert_eq!(
            create_host_param!({
                name -> "test-host".to_string()
            }),
            CreateHostParam {
                name: "test-host".to_string(),
                ..Default::default()
            }
        );

        assert_eq!(
            create_host_param!({
                name -> "test-host".to_string()
                display_name -> Some("test-host".to_string())
            }),
            CreateHostParam {
                name: "test-host".to_string(),
                display_name: Some("test-host".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn serialize_create_host_param() {
        assert_eq!(
            serde_json::to_value(&create_host_param_example()).unwrap(),
            json_example()
        );
    }

    #[test]
    fn deserialize_create_host_param() {
        assert_eq!(
            create_host_param_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }
}

impl client::Client {
    /// Creates a new host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#create.
    pub fn create_host(&self, param: CreateHostParam) -> Result<String> {
        self.request(
            Method::POST,
            "/api/v0/hosts",
            vec![],
            Some(param),
            |host_id: HostId| host_id.id,
        )
    }
}
