use crate::client;
use crate::entity::Id;
use crate::errors::*;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::default::Default;
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
    pub async fn get_host(&self, id: HostId) -> Result<Host> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}", id),
            vec![],
            client::empty_body(),
            |res: GetHostResponse| res.host,
        )
        .await
    }

    /// Updates a host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#update-information.
    pub async fn update_host(&self, id: HostId, host: HostValue) -> Result<HostId> {
        self.request(
            Method::PUT,
            format!("/api/v0/hosts/{}", id),
            vec![],
            Some(host),
            |res: CreateHostResponse| res.id,
        )
        .await
    }

    /// Updates host status.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#update-status.
    pub async fn update_host_status(&self, id: HostId, status: HostStatus) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/{}/status", id),
            vec![],
            Some(HashMap::<_, _>::from_iter([("status", status)])),
            |_: HashMap<String, bool>| (),
        )
        .await
    }

    /// Updates host roles.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#update-roles.
    pub async fn update_host_roles(&self, id: HostId, role_fullnames: Vec<String>) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/{}/role-fullnames", id),
            vec![],
            Some(HashMap::<_, _>::from_iter([(
                "roleFullnames",
                role_fullnames,
            )])),
            |_: HashMap<String, bool>| (),
        )
        .await
    }

    /// Retires a host.
    ///
    /// See https://mackerel.io/api-docs/entry/hosts#retire.
    pub async fn retire_host(&self, id: HostId) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/{}/retire", id),
            vec![],
            client::empty_body(),
            |_: HashMap<String, bool>| (),
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
    pub async fn list_host_metric_names(&self, id: HostId) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metric-names", id),
            vec![],
            client::empty_body(),
            |res: ListMetricNamesResponse| res.names,
        )
        .await
    }
}
