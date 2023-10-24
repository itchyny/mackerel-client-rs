use crate::client;
use crate::entity::Id;
use crate::error::*;
use crate::role::RoleName;
use crate::service::ServiceName;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use url::form_urlencoded;

/// A host
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: HostId,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub size: HostSize,
    pub status: HostStatus,
    pub is_retired: bool,
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub retired_at: Option<DateTime<Utc>>,
    pub roles: HashMap<ServiceName, Vec<RoleName>>,
    #[serde(flatten)]
    pub value: HostValue,
}

/// Host size
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HostSize {
    Standard,
    Micro,
}

impl fmt::Display for HostSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HostSize::Standard => write!(f, "standard"),
            HostSize::Micro => write!(f, "micro"),
        }
    }
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
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostValue {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
    pub meta: HashMap<String, Value>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
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

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInterface {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<Ipv4Addr>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<Ipv6Addr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Ipv4Addr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<Ipv6Addr>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostCheck {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use crate::host::*;
    use serde_json::json;

    fn host_example1() -> Host {
        Host {
            id: "abcde1".into(),
            created_at: DateTime::from_timestamp(1700000000, 0).unwrap(),
            size: HostSize::Standard,
            status: HostStatus::Working,
            is_retired: false,
            retired_at: None,
            roles: HashMap::new(),
            value: HostValue {
                name: "example-host".to_string(),
                display_name: None,
                custom_identifier: None,
                meta: HashMap::new(),
                memo: "".to_string(),
                interfaces: vec![],
                role_fullnames: vec![],
                checks: vec![],
            },
        }
    }

    fn host_json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "createdAt": 1700000000,
            "size": "standard",
            "status": "working",
            "isRetired": false,
            "roles": {},
            "name": "example-host",
            "meta": {},
        })
    }

    fn host_example2() -> Host {
        Host {
            id: "abcde2".into(),
            created_at: DateTime::from_timestamp(1700000000, 0).unwrap(),
            size: HostSize::Micro,
            status: HostStatus::Poweroff,
            is_retired: true,
            retired_at: Some(DateTime::from_timestamp(1710000000, 0).unwrap()),
            roles: HashMap::<_, _>::from_iter([(
                "ExampleService".into(),
                vec!["ExampleRole".into()],
            )]),
            value: HostValue {
                name: "example-host".to_string(),
                display_name: Some("Example host".to_string()),
                custom_identifier: Some("custom-identifier".to_string()),
                meta: HashMap::new(),
                memo: "host memo".to_string(),
                interfaces: vec![HostInterface {
                    name: "lo0".to_string(),
                    ipv4_addresses: vec!["127.0.0.1".parse().unwrap()],
                    ipv6_addresses: vec!["fe80::1".parse().unwrap()],
                    ..HostInterface::default()
                }],
                role_fullnames: vec!["ExampleService:ExampleRole".to_string()],
                checks: vec![
                    HostCheck {
                        name: "check0".to_string(),
                        memo: "check0 memo".to_string(),
                    },
                    HostCheck {
                        name: "check1".to_string(),
                        memo: "".to_string(),
                    },
                ],
            },
        }
    }

    fn host_json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "createdAt": 1700000000,
            "size": "micro",
            "status": "poweroff",
            "isRetired": true,
            "retiredAt": 1710000000,
            "roles": {"ExampleService": ["ExampleRole"]},
            "name": "example-host",
            "displayName": "Example host",
            "customIdentifier": "custom-identifier",
            "meta": {},
            "memo": "host memo",
            "interfaces": [
                {
                    "name": "lo0",
                    "ipv4Addresses": ["127.0.0.1"],
                    "ipv6Addresses": ["fe80::1"],
                },
            ],
            "roleFullnames": ["ExampleService:ExampleRole"],
            "checks": [{"name": "check0", "memo": "check0 memo"}, {"name": "check1"}],
        })
    }

    #[test]
    fn serialize_host() {
        assert_eq!(
            serde_json::to_value(&host_example1()).unwrap(),
            host_json_example1()
        );
        assert_eq!(
            serde_json::to_value(&host_example2()).unwrap(),
            host_json_example2()
        );
    }

    #[test]
    fn deserialize_host() {
        assert_eq!(
            host_example1(),
            serde_json::from_value(host_json_example1()).unwrap()
        );
        assert_eq!(
            host_example2(),
            serde_json::from_value(host_json_example2()).unwrap()
        );
    }

    #[test]
    fn test_host_sizes() {
        let test_cases = [(HostSize::Standard, "standard"), (HostSize::Micro, "micro")];
        for &(host_size, status_str) in &test_cases {
            let str_value = serde_json::Value::String(status_str.to_string());
            assert_eq!(
                host_size,
                serde_json::from_value(str_value.clone()).unwrap()
            );
            assert_eq!(str_value, serde_json::to_value(host_size).unwrap());
            assert_eq!(str_value, format!("{}", host_size).as_str());
        }
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
struct GetHostByCustomIdentifierResponse {
    host: Option<Host>,
}

#[derive(Deserialize)]
struct ListHostsResponse {
    hosts: Vec<Host>,
}

#[derive(Serialize)]
struct UpdateHostStatusesRequest {
    ids: Vec<HostId>,
    status: HostStatus,
}

#[derive(Deserialize)]
struct ListMetricNamesResponse {
    names: Vec<String>,
}

impl client::Client {
    /// Creates a new host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#create>.
    pub async fn create_host(&self, host_value: HostValue) -> Result<HostId> {
        self.request(
            Method::POST,
            "/api/v0/hosts",
            vec![],
            Some(host_value),
            |res: CreateHostResponse| res.id,
        )
        .await
    }

    /// Gets a host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#get>.
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

    /// Gets a host by the custom identifier.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#get-by-custom-identifier>.
    pub async fn get_host_by_custom_identifier(
        &self,
        custom_identifier: String,
    ) -> Result<Option<Host>> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/hosts-by-custom-identifier/{}",
                form_urlencoded::byte_serialize(custom_identifier.as_bytes()).collect::<String>(),
            ),
            vec![],
            client::empty_body(),
            |res: GetHostByCustomIdentifierResponse| res.host,
        )
        .await
    }

    /// Updates a host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-information>.
    pub async fn update_host(&self, host_id: HostId, host_value: HostValue) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/hosts/{}", host_id),
            vec![],
            Some(host_value),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Updates host status.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-status>.
    pub async fn update_host_status(&self, host_id: HostId, host_status: HostStatus) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/{}/status", host_id),
            vec![],
            Some(HashMap::from([("status", host_status)])),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Updates host statuses.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-status>.
    pub async fn update_host_statuses(
        &self,
        host_ids: Vec<HostId>,
        host_status: HostStatus,
    ) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/bulk-update-statuses"),
            vec![],
            Some(UpdateHostStatusesRequest {
                ids: host_ids,
                status: host_status,
            }),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Updates host roles.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-roles>.
    pub async fn update_host_roles(
        &self,
        host_id: HostId,
        role_fullnames: Vec<String>,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/hosts/{}/role-fullnames", host_id),
            vec![],
            Some(HashMap::from([("roleFullnames", role_fullnames)])),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Retires a host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#retire>.
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

    /// Retires hosts.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#bulk-retire>.
    pub async fn retire_hosts(&self, host_ids: Vec<HostId>) -> Result<()> {
        self.request(
            Method::POST,
            format!("/api/v0/hosts/bulk-retire"),
            vec![],
            Some(HashMap::from([("ids", host_ids)])),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Fetches hosts.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#list>.
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
    /// See <https://mackerel.io/api-docs/entry/hosts#metric-names>.
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
