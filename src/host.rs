use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;
use url::form_urlencoded;

use crate::alert::AlertStatus;
use crate::client::*;
use crate::entity::Id;
use crate::error::Result;
use crate::monitor::{MonitorId, MonitorType};
use crate::role::{RoleFullname, RoleName};
use crate::service::ServiceName;

/// A host entity
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub id: HostId,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[builder(default)]
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
    PartialEq,
    Eq,
    Copy,
    Clone,
    Debug,
    Default,
    Display,
    EnumString,
    SerializeDisplay,
    DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum HostSize {
    #[default]
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
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_identifier: Option<String>,
    #[builder(default)]
    pub meta: HashMap<String, Value>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interfaces: Vec<HostInterface>,
    #[builder(
        default,
        setter(transform = |role_fullnames: impl IntoIterator<Item = impl Into<RoleFullname>>| role_fullnames
            .into_iter().map(|role_fullname| role_fullname.into()).collect::<Vec<_>>()),
    )]
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
    #[builder(
        default,
        setter(transform = |ipv4_addresses: impl IntoIterator<Item = impl Into<Ipv4Addr>>| ipv4_addresses
            .into_iter().map(|ipv4_address| ipv4_address.into()).collect::<Vec<_>>()),
    )]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<Ipv4Addr>,
    #[builder(
        default,
        setter(transform = |ipv6_addresses: impl IntoIterator<Item = impl Into<Ipv6Addr>>| ipv6_addresses
            .into_iter().map(|ipv6_address| ipv6_address.into()).collect::<Vec<_>>()),
    )]
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

#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct MonitoredStatus {
    pub monitor_id: MonitorId,
    pub status: AlertStatus,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<MonitoredStatusDetail>,
}

#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct MonitoredStatusDetail {
    #[serde(rename = "type")]
    pub monitor_type: MonitorType,
    pub message: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn host_example1() -> Host {
        Host::builder()
            .id("host1")
            .created_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .status(HostStatus::Working)
            .value(HostValue::builder().name("example-host").build())
            .build()
    }

    fn host_json_example1() -> serde_json::Value {
        json!({
            "id": "host1",
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
            .id("host2")
            .created_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .size(HostSize::Micro)
            .status(HostStatus::Poweroff)
            .is_retired(true)
            .retired_at(DateTime::from_timestamp(1710000000, 0).unwrap())
            .roles([("service0".into(), vec!["role0".into()])])
            .value(
                HostValue::builder()
                    .name("example-host")
                    .display_name("Example host")
                    .custom_identifier("custom-identifier")
                    .meta([("agent-name".to_string(), json!("mackerel-agent"))])
                    .memo("host memo")
                    .interfaces([HostInterface::builder()
                        .name("lo0")
                        .mac_address("00:00:00:00:00:00")
                        .ipv4_addresses([[127, 0, 0, 1]])
                        .ipv6_addresses([[0xfe80, 0, 0, 0, 0, 0, 0, 1]])
                        .ip_address([127, 0, 0, 1])
                        .ipv6_address([0xfe80, 0, 0, 0, 0, 0, 0, 1])
                        .build()])
                    .role_fullnames(["service0:role0"])
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
            "id": "host2",
            "createdAt": 1700000000,
            "size": "micro",
            "status": "poweroff",
            "isRetired": true,
            "retiredAt": 1710000000,
            "roles": {"service0": ["role0"]},
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
            "roleFullnames": ["service0:role0"],
            "checks": [{"name": "check0", "memo": "memo"}, {"name": "check1"}],
        })
    }

    #[rstest]
    #[case(host_example1(), host_json_example1())]
    #[case(host_example2(), host_json_example2())]
    fn test_host_json(#[case] host: Host, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&host).unwrap(), json);
        assert_eq!(host, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(HostSize::Standard, "standard")]
    #[case(HostSize::Micro, "micro")]
    #[case(HostSize::default(), "standard")]
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

    fn monitored_status_example1() -> MonitoredStatus {
        MonitoredStatus::builder()
            .monitor_id("monitor0")
            .status(AlertStatus::Ok)
            .build()
    }

    fn monitored_status_json_example1() -> serde_json::Value {
        json!({
            "monitorId": "monitor0",
            "status": "OK",
        })
    }

    fn monitored_status_example2() -> MonitoredStatus {
        MonitoredStatus::builder()
            .monitor_id("monitor0")
            .status(AlertStatus::Critical)
            .detail(
                MonitoredStatusDetail::builder()
                    .monitor_type(MonitorType::Check)
                    .message("This is a check monitoring message.")
                    .memo("This is a check monitoring memo.")
                    .build(),
            )
            .build()
    }

    fn monitored_status_json_example2() -> serde_json::Value {
        json!({
            "monitorId": "monitor0",
            "status": "CRITICAL",
            "detail": {
                "type": "check",
                "message": "This is a check monitoring message.",
                "memo": "This is a check monitoring memo.",
            },
        })
    }

    #[rstest]
    #[case(monitored_status_example1(), monitored_status_json_example1())]
    #[case(monitored_status_example2(), monitored_status_json_example2())]
    fn test_monitored_statuses(
        #[case] monitored_status: MonitoredStatus,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&monitored_status).unwrap(), json);
        assert_eq!(monitored_status, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Creates a new host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#create>.
    pub async fn create_host(&self, host_value: &HostValue) -> Result<HostId> {
        self.request(
            Method::POST,
            "/api/v0/hosts",
            query_params![],
            request_body!(host_value),
            response_body! { id: HostId },
        )
        .await
    }

    /// Gets a host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#get>.
    pub async fn get_host(&self, host_id: impl Into<HostId>) -> Result<Host> {
        self.request(
            Method::GET,
            format_url!("/api/v0/hosts/{}", host_id),
            query_params![],
            request_body![],
            response_body! { host: Host },
        )
        .await
    }

    /// Gets a host by the custom identifier.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#get-by-custom-identifier>.
    pub async fn get_host_by_custom_identifier(
        &self,
        custom_identifier: impl AsRef<str>,
    ) -> Result<Option<Host>> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/hosts-by-custom-identifier/{}",
                form_urlencoded::byte_serialize(custom_identifier.as_ref().as_bytes())
                    .collect::<String>(),
            ),
            query_params![],
            request_body![],
            response_body! { host: Option<Host> },
        )
        .await
    }

    /// Updates a host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-information>.
    pub async fn update_host(
        &self,
        host_id: impl Into<HostId>,
        host_value: &HostValue,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/hosts/{}", host_id),
            query_params![],
            request_body!(host_value),
            response_body!(),
        )
        .await
    }

    /// Updates host status.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-status>.
    pub async fn update_host_status(
        &self,
        host_id: impl Into<HostId>,
        host_status: HostStatus,
    ) -> Result<()> {
        self.request(
            Method::POST,
            format_url!("/api/v0/hosts/{}/status", host_id),
            query_params![],
            request_body! { status: HostStatus = host_status },
            response_body!(),
        )
        .await
    }

    /// Updates host statuses.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-status>.
    pub async fn update_host_statuses(
        &self,
        host_ids: impl IntoIterator<Item = impl Into<HostId>>,
        host_status: HostStatus,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/hosts/bulk-update-statuses",
            query_params![],
            request_body! {
                ids: Vec<HostId> = host_ids
                    .into_iter()
                    .map(|host_id| host_id.into())
                    .collect::<Vec<_>>(),
                status: HostStatus = host_status,
            },
            response_body!(),
        )
        .await
    }

    /// Updates host roles.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#update-roles>.
    pub async fn update_host_roles(
        &self,
        host_id: impl Into<HostId>,
        role_fullnames: impl IntoIterator<Item = impl Into<RoleFullname>>,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/hosts/{}/role-fullnames", host_id),
            query_params![],
            request_body! {
                roleFullnames: Vec<RoleFullname> = role_fullnames
                    .into_iter()
                    .map(|role_fullname| role_fullname.into())
                    .collect::<Vec<_>>(),
            },
            response_body!(),
        )
        .await
    }

    /// Retires a host.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#retire>.
    pub async fn retire_host(&self, host_id: impl Into<HostId>) -> Result<()> {
        self.request(
            Method::POST,
            format_url!("/api/v0/hosts/{}/retire", host_id),
            query_params![],
            request_body![],
            response_body!(),
        )
        .await
    }

    /// Retires hosts.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#bulk-retire>.
    pub async fn retire_hosts(
        &self,
        host_ids: impl IntoIterator<Item = impl Into<HostId>>,
    ) -> Result<()> {
        self.request(
            Method::POST,
            "/api/v0/hosts/bulk-retire",
            query_params![],
            request_body! {
                ids: Vec<HostId> = host_ids
                    .into_iter()
                    .map(|host_id| host_id.into())
                    .collect::<Vec<_>>(),
            },
            response_body!(),
        )
        .await
    }

    /// Fetches hosts in the organization.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#list>.
    pub async fn list_hosts(&self) -> Result<Vec<Host>> {
        self.list_hosts_internal(query_params![]).await
    }

    /// Fetches hosts in a service.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#list>.
    pub async fn list_service_hosts(
        &self,
        service_name: impl Into<ServiceName>,
    ) -> Result<Vec<Host>> {
        self.list_hosts_internal(query_params! {
            service = service_name.into().to_string(),
        })
        .await
    }

    /// Fetches hosts in roles.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#list>.
    pub async fn list_role_hosts(
        &self,
        service_name: impl Into<ServiceName>,
        role_names: impl IntoIterator<Item = impl Into<RoleName>>,
    ) -> Result<Vec<Host>> {
        self.list_hosts_internal(
            &std::iter::once(("service", service_name.into().to_string()))
                .chain(
                    role_names
                        .into_iter()
                        .map(|role_name| ("role", role_name.into().to_string())),
                )
                .collect::<Vec<_>>(),
        )
        .await
    }

    async fn list_hosts_internal(
        &self,
        query_params: &[(&str, impl AsRef<str>)],
    ) -> Result<Vec<Host>> {
        self.request(
            Method::GET,
            "/api/v0/hosts",
            query_params,
            request_body![],
            response_body! { hosts: Vec<Host> },
        )
        .await
    }

    /// Fetches host metric names.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#metric-names>.
    pub async fn list_host_metric_names(&self, host_id: impl Into<HostId>) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format_url!("/api/v0/hosts/{}/metric-names", host_id),
            query_params![],
            request_body![],
            response_body! { names: Vec<String> },
        )
        .await
    }

    /// Fetches host monitoring statuses.
    ///
    /// See <https://mackerel.io/api-docs/entry/hosts#monitored-statuses>.
    pub async fn list_host_monitored_statuses(
        &self,
        host_id: impl Into<HostId>,
    ) -> Result<Vec<MonitoredStatus>> {
        self.request(
            Method::GET,
            format_url!("/api/v0/hosts/{}/monitored-statuses", host_id),
            query_params![],
            request_body![],
            response_body! { monitoredStatuses: Vec<MonitoredStatus> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use chrono::DateTime;
    use serde_json::json;

    use crate::alert::*;
    use crate::host::*;
    use crate::monitor::*;
    use crate::tests::*;

    fn value_example() -> HostValue {
        HostValue::builder()
            .name("example-host")
            .display_name("Example host")
            .custom_identifier("custom-identifier")
            .meta([("agent-name".to_string(), json!("mackerel-agent"))])
            .memo("This is a host memo.")
            .build()
    }

    fn entity_example() -> Host {
        Host::builder()
            .id("host0")
            .created_at(DateTime::from_timestamp(1700000000, 0).unwrap())
            .status(HostStatus::Working)
            .value(value_example())
            .build()
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "name": "example-host",
            "displayName": "Example host",
            "customIdentifier": "custom-identifier",
            "meta": { "agent-name": "mackerel-agent" },
            "memo": "This is a host memo.",
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("host0");
        json["createdAt"] = json!(1700000000);
        json["size"] = json!("standard");
        json["status"] = json!("working");
        json["isRetired"] = json!(false);
        json["roles"] = json!({});
        json
    }

    #[async_std::test]
    async fn create_host() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/hosts",
            request = value_json_example(),
            response = json!({ "id": "host0" }),
        };
        assert_eq!(
            test_client!(server).create_host(&value_example()).await,
            Ok(HostId::from("host0")),
        );
    }

    #[async_std::test]
    async fn get_host() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts/host0",
            response = json!({ "host": entity_json_example() }),
        };
        assert_eq!(
            test_client!(server).get_host("host0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server).get_host(HostId::from("host0")).await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn get_host_by_custom_identifier() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts-by-custom-identifier/test%2Fcustom%3Fidentifier%26",
            response = json!({ "host": entity_json_example() }),
        };
        assert_eq!(
            test_client!(server)
                .get_host_by_custom_identifier("test/custom?identifier&")
                .await,
            Ok(Some(entity_example())),
        );
    }

    #[async_std::test]
    async fn get_host_by_custom_identifier_not_found() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts-by-custom-identifier/not-found",
            response = json!({ "host": None::<()> }),
        };
        assert_eq!(
            test_client!(server)
                .get_host_by_custom_identifier(String::from("not-found"))
                .await,
            Ok(None),
        );
    }

    #[async_std::test]
    async fn update_host() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/hosts/host0",
            request = value_json_example(),
            response = json!({ "id": "host0" }),
        };
        assert_eq!(
            test_client!(server)
                .update_host("host0", &value_example())
                .await,
            Ok(()),
        );
        assert_eq!(
            test_client!(server)
                .update_host(HostId::from("host0"), &value_example())
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn update_host_status() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/hosts/host0/status",
            request = json!({ "status": "standby" }),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .update_host_status("host0", HostStatus::Standby)
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn update_host_statuses() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/hosts/bulk-update-statuses",
            request = json!({
                "ids": ["host0", "host1", "host2"],
                "status": "standby",
            }),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .update_host_statuses(["host0", "host1", "host2"], HostStatus::Standby)
                .await,
            Ok(()),
        );
        assert_eq!(
            test_client!(server)
                .update_host_statuses(
                    vec![
                        HostId::from("host0"),
                        HostId::from("host1"),
                        HostId::from("host2")
                    ],
                    HostStatus::Standby
                )
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn update_host_roles() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/hosts/host0/role-fullnames",
            request = json!({ "roleFullnames": ["service0:role0"] }),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .update_host_roles("host0", ["service0:role0"])
                .await,
            Ok(()),
        );
        assert_eq!(
            test_client!(server)
                .update_host_roles(
                    HostId::from("host0"),
                    vec![RoleFullname::from("service0:role0")]
                )
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn retire_host() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/hosts/host0/retire",
            response = json!({ "success": true }),
        };
        assert_eq!(test_client!(server).retire_host("host0").await, Ok(()));
        assert_eq!(
            test_client!(server)
                .retire_host(HostId::from("host0"))
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn retire_hosts() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/hosts/bulk-retire",
            request = json!({ "ids": ["host0", "host1", "host2"] }),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .retire_hosts(["host0", "host1", "host2"])
                .await,
            Ok(()),
        );
        assert_eq!(
            test_client!(server)
                .retire_hosts(vec![
                    HostId::from("host0"),
                    HostId::from("host1"),
                    HostId::from("host2")
                ])
                .await,
            Ok(()),
        );
    }

    #[async_std::test]
    async fn list_hosts() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts",
            response = json!({
                "hosts": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_hosts().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn list_service_hosts() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts",
            query_params = "service=service0",
            response = json!({
                "hosts": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_service_hosts("service0").await,
            Ok(vec![entity_example()]),
        );
        assert_eq!(
            test_client!(server)
                .list_service_hosts(ServiceName::from("service0"))
                .await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn list_role_hosts() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts",
            query_params = "service=service0&role=role0&role=role1&role=role2",
            response = json!({
                "hosts": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_role_hosts("service0", ["role0", "role1", "role2"])
                .await,
            Ok(vec![entity_example()]),
        );
        assert_eq!(
            test_client!(server)
                .list_role_hosts(
                    ServiceName::from("service0"),
                    vec![
                        RoleName::from("role0"),
                        RoleName::from("role1"),
                        RoleName::from("role2"),
                    ]
                )
                .await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn list_host_metric_names() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts/host0/metric-names",
            response = json!({
                "names": [
                    "custom.host.metric0",
                    "custom.host.metric1",
                    "custom.host.metric2",
                ],
            }),
        };
        assert_eq!(
            test_client!(server).list_host_metric_names("host0").await,
            Ok(vec![
                "custom.host.metric0".to_owned(),
                "custom.host.metric1".to_owned(),
                "custom.host.metric2".to_owned(),
            ]),
        );
        assert_eq!(
            test_client!(server)
                .list_host_metric_names(HostId::from("host0"))
                .await,
            Ok(vec![
                "custom.host.metric0".to_owned(),
                "custom.host.metric1".to_owned(),
                "custom.host.metric2".to_owned(),
            ]),
        );
    }

    #[async_std::test]
    async fn list_host_monitored_statuses() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts/host0/monitored-statuses",
            response = json!({
                "monitoredStatuses": [
                    {
                        "monitorId": "monitor0",
                        "status": "OK",
                        "detail": {
                            "type": "check",
                            "message": "This is a check monitoring message.",
                        },
                    },
                ],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_host_monitored_statuses("host0")
                .await,
            Ok(vec![MonitoredStatus::builder()
                .monitor_id("monitor0")
                .status(AlertStatus::Ok)
                .detail(
                    MonitoredStatusDetail::builder()
                        .monitor_type(MonitorType::Check)
                        .message("This is a check monitoring message.")
                        .build()
                )
                .build()]),
        );
    }
}
