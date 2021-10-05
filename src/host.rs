use crate::client;
use crate::errors::*;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::{map::Map, Value};
use serde_with::skip_serializing_none;
use std::default;

// TODO: Define its fields.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface();

// TODO: Type as struct
pub type Meta = Map<String, Value>;
pub type Checks = Map<String, Value>;

// Input type for POST /api/v0/hosts
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHostParam {
    pub name: String,
    pub display_name: Option<String>,
    pub custom_identifier: Option<String>,
    pub meta: Meta,
    pub interfaces: Option<Vec<Interface>>,
    pub role_fullnames: Option<Vec<String>>,
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
    use serde_json::json;

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

#[derive(Deserialize)]
struct CreateHostResponse {
    id: String,
}

impl client::Client {
    /// Creates a new host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#create.
    pub async fn create_host(&self, param: CreateHostParam) -> Result<String> {
        self.request(
            Method::POST,
            "/api/v0/hosts",
            vec![],
            Some(param),
            |res: CreateHostResponse| res.id,
        )
        .await
    }
}
