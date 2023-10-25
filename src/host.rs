use crate::client;
use crate::entity::Id;
use crate::error::*;
use crate::role::{RoleFullname, RoleName};
use crate::service::ServiceName;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;
use url::form_urlencoded;

/// A host
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: HostId,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub size: HostSize,
    pub status: HostStatus,
    #[builder(default)]
    pub is_retired: bool,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub retired_at: Option<DateTime<Utc>>,
    #[builder(default)]
    pub roles: HashMap<ServiceName, Vec<RoleName>>,
    #[serde(flatten)]
    pub value: HostValue,
}

impl std::ops::Deref for Host {
    type Target = HostValue;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Host size
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum HostSize {
    Standard,
    Micro,
}

/// Host status
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum HostStatus {
    Working,
    Standby,
    Maintenance,
    Poweroff,
}

/// A host id
pub type HostId = Id<HostValue>;

/// A host value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HostValue {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub custom_identifier: Option<String>,
    #[builder(default)]
    pub meta: HashMap<String, Value>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<HostInterface>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_fullnames: Vec<RoleFullname>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<HostCheck>,
}

#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HostInterface {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<Ipv4Addr>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<Ipv6Addr>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Ipv4Addr>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<Ipv6Addr>,
}

#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HostCheck {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use crate::host::*;
    use rstest::rstest;
    use serde_json::json;

    fn host_example1() -> Host {
        Host::builder()
            .id("abcde1")
            .created_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .size(HostSize::Standard)
            .status(HostStatus::Working)
            .value(HostValue::builder().name("example-host").build())
            .build()
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
        Host::builder()
            .id("abcde2")
            .created_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .size(HostSize::Micro)
            .status(HostStatus::Poweroff)
            .is_retired(true)
            .retired_at(DateTime::from_timestamp(1710000000, 0).unwrap())
            .roles([("ExampleService".into(), vec!["ExampleRole".into()])])
            .value(
                HostValue::builder()
                    .name("example-host")
                    .display_name("Example host")
                    .custom_identifier("custom-identifier")
                    .meta([(
                        "agent-name".to_string(),
                        serde_json::to_value("mackerel-agent").unwrap(),
                    )])
                    .memo("host memo")
                    .interfaces([HostInterface::builder()
                        .name("lo0")
                        .mac_address("00:00:00:00:00:00")
                        .ipv4_addresses([[127, 0, 0, 1].into()])
                        .ipv6_addresses([[0xfe80, 0, 0, 0, 0, 0, 0, 1].into()])
                        .ip_address([127, 0, 0, 1])
                        .ipv6_address([0xfe80, 0, 0, 0, 0, 0, 0, 1])
                        .build()])
                    .role_fullnames(["ExampleService:ExampleRole".into()])
                    .checks([
                        HostCheck::builder().name("check0").memo("memo").build(),
                        HostCheck::builder().name("check1").build(),
                    ])
                    .build(),
            )
            .build()
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
            "meta": {"agent-name": "mackerel-agent"},
            "memo": "host memo",
            "interfaces": [
                {
                    "name": "lo0",
                    "macAddress": "00:00:00:00:00:00",
                    "ipv4Addresses": ["127.0.0.1"],
                    "ipv6Addresses": ["fe80::1"],
                    "ipAddress": "127.0.0.1",
                    "ipv6Address": "fe80::1",
                },
            ],
            "roleFullnames": ["ExampleService:ExampleRole"],
            "checks": [{"name": "check0", "memo": "memo"}, {"name": "check1"}],
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

    #[rstest]
    #[case(HostSize::Standard, "standard")]
    #[case(HostSize::Micro, "micro")]
    fn test_host_size(#[case] host_size: HostSize, #[case] host_size_str: &str) {
        assert_eq!(host_size.to_string(), host_size_str);
        assert_eq!(host_size, host_size_str.parse().unwrap());
        assert_eq!(
            host_size,
            serde_json::from_value(host_size_str.into()).unwrap()
        );
        assert_eq!(serde_json::to_value(host_size).unwrap(), host_size_str);
    }

    #[rstest]
    #[case(HostStatus::Working, "working")]
    #[case(HostStatus::Standby, "standby")]
    #[case(HostStatus::Maintenance, "maintenance")]
    #[case(HostStatus::Poweroff, "poweroff")]
    fn test_host_status(#[case] host_status: HostStatus, #[case] host_status_str: &str) {
        assert_eq!(host_status.to_string(), host_status_str);
        assert_eq!(host_status, host_status_str.parse().unwrap());
        assert_eq!(
            host_status,
            serde_json::from_value(host_status_str.into()).unwrap()
        );
        assert_eq!(serde_json::to_value(host_status).unwrap(), host_status_str);
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
        role_fullnames: Vec<RoleFullname>,
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
