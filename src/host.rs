use crate::client;
use crate::entity::Id;
use crate::error::*;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::iter::FromIterator;

/// A host
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: HostId,
    pub created_at: u64,
    pub size: String,
    pub status: HostStatus,
    pub memo: String,
    pub is_retired: bool,
    pub retired_at: Option<u64>,
    pub roles: HashMap<String, Vec<String>>,
    #[serde(flatten)]
    pub value: HostValue,
}

/// Host status
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostStatus {
    Working,
    Standby,
    Maintenance,
    Poweroff,
}

impl fmt::Display for HostStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HostStatus::Working => write!(f, "working"),
            HostStatus::Standby => write!(f, "standby"),
            HostStatus::Maintenance => write!(f, "maintenance"),
            HostStatus::Poweroff => write!(f, "poweroff"),
        }
    }
}

/// A host id
pub type HostId = Id<HostValue>;

/// A host value
#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HostValue {
    pub name: String,
    pub display_name: Option<String>,
    pub custom_identifier: Option<String>,
    pub meta: HashMap<String, Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<HostInterface>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_fullnames: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<HostCheck>,
}

impl std::ops::Deref for Host {
    type Target = HostValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[skip_serializing_none]
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInterface {
    pub name: String,
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<String>,
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

    #[test]
    fn test_host_statuses() {
        let test_cases = [
            (HostStatus::Working, "working"),
            (HostStatus::Standby, "standby"),
            (HostStatus::Maintenance, "maintenance"),
            (HostStatus::Poweroff, "poweroff"),
        ];
        for &(host_status, status_str) in &test_cases {
            let str_value = serde_json::Value::String(status_str.to_string());
            assert_eq!(
                host_status,
                serde_json::from_value(str_value.clone()).unwrap()
            );
            assert_eq!(str_value, serde_json::to_value(host_status).unwrap());
            assert_eq!(str_value, format!("{}", host_status).as_str());
        }
    }
}

#[derive(Deserialize)]
struct CreateHostResponse {
    id: HostId,
}

#[derive(Deserialize)]
struct GetHostResponse {
    host: Host,
}

#[derive(Deserialize)]
struct ListHostsResponse {
    hosts: Vec<Host>,
}

#[derive(Deserialize)]
struct ListMetricNamesResponse {
    names: Vec<String>,
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

    /// Gets a host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#get.
    pub async fn get_host(&self, host_id: HostId) -> Result<Host> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}", host_id),
            vec![],
            client::empty_body(),
            |res: GetHostResponse| res.host,
        )
        .await
    }

    /// Updates a host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#update-information.
    pub async fn update_host(&self, host_id: HostId, host: HostValue) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/hosts/{}", host_id),
            vec![],
            Some(host),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Updates host status.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#update-status.
    pub async fn update_host_status(&self, host_id: HostId, status: HostStatus) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/{}/status", host_id),
            vec![],
            Some(HashMap::<_, _>::from_iter([("status", status)])),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Updates host roles.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#update-roles.
    pub async fn update_host_roles(
        &self,
        host_id: HostId,
        role_fullnames: Vec<String>,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/hosts/{}/role-fullnames", host_id),
            vec![],
            Some(HashMap::<_, _>::from_iter([(
                "roleFullnames",
                role_fullnames,
            )])),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Retires a host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#retire.
    pub async fn retire_host(&self, host_id: HostId) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/{}/retire", host_id),
            vec![],
            client::empty_body(),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Fetches hosts.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#list.
    pub async fn list_hosts(&self) -> Result<Vec<Host>> {
        self.request(
            Method::GET,
            "/api/v0/hosts",
            vec![],
            client::empty_body(),
            |res: ListHostsResponse| res.hosts,
        )
        .await
    }

    /// Fetches host metric names.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#metric-names.
    pub async fn list_host_metric_names(&self, host_id: HostId) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metric-names", host_id),
            vec![],
            client::empty_body(),
            |res: ListMetricNamesResponse| res.names,
        )
        .await
    }
}
