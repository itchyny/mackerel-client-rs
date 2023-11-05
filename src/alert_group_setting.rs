use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::monitor::MonitorId;
use crate::role::RoleFullname;
use crate::service::ServiceName;

/// An alert group setting entity
pub type AlertGroupSetting = Entity<AlertGroupSettingValue>;

/// An alert group setting id
pub type AlertGroupSettingId = Id<AlertGroupSettingValue>;

/// An alert group setting value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AlertGroupSettingValue {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_scopes: Vec<ServiceName>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_scopes: Vec<RoleFullname>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_scopes: Vec<MonitorId>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn alert_group_setting_example1() -> AlertGroupSetting {
        AlertGroupSetting::builder()
            .id("setting1")
            .value(
                AlertGroupSettingValue::builder()
                    .name("Example alert group setting")
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "setting1",
            "name": "Example alert group setting",
        })
    }

    fn alert_group_setting_example2() -> AlertGroupSetting {
        AlertGroupSetting::builder()
            .id("setting2")
            .value(
                AlertGroupSettingValue::builder()
                    .name("Example alert group setting")
                    .memo("This is an alert group setting memo.")
                    .service_scopes(["service0".into()])
                    .role_scopes(["service1:role1".into()])
                    .monitor_scopes(["monitor0".into()])
                    .notification_interval(60)
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "setting2",
            "name": "Example alert group setting",
            "memo": "This is an alert group setting memo.",
            "serviceScopes": ["service0"],
            "roleScopes": ["service1:role1"],
            "monitorScopes": ["monitor0"],
            "notificationInterval": 60,
        })
    }

    #[rstest]
    #[case(alert_group_setting_example1(), json_example1())]
    #[case(alert_group_setting_example2(), json_example2())]
    fn test_alert_group_setting(
        #[case] alert_group_setting: AlertGroupSetting,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&alert_group_setting).unwrap(), json);
        assert_eq!(alert_group_setting, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Fetches all the alert group settings.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#list>.
    pub async fn list_alert_group_settings(&self) -> Result<Vec<AlertGroupSetting>> {
        self.request(
            Method::GET,
            "/api/v0/alert-group-settings",
            query_params![],
            request_body![],
            response_body! { alertGroupSettings: Vec<AlertGroupSetting> },
        )
        .await
    }

    /// Creates a new alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#create>.
    pub async fn create_alert_group_setting(
        &self,
        alert_group_setting_value: &AlertGroupSettingValue,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::POST,
            "/api/v0/alert-group-settings",
            query_params![],
            request_body!(alert_group_setting_value),
            response_body!(..),
        )
        .await
    }

    /// Gets an alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#get>.
    pub async fn get_alert_group_setting(
        &self,
        alert_group_setting_id: impl Into<AlertGroupSettingId>,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::GET,
            format_url!("/api/v0/alert-group-settings/{}", alert_group_setting_id),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Updates an alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#update>.
    pub async fn update_alert_group_setting(
        &self,
        alert_group_setting_id: impl Into<AlertGroupSettingId>,
        alert_group_setting_value: &AlertGroupSettingValue,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/alert-group-settings/{}", alert_group_setting_id),
            query_params![],
            request_body!(alert_group_setting_value),
            response_body!(..),
        )
        .await
    }

    /// Deletes an alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#delete>.
    pub async fn delete_alert_group_setting(
        &self,
        alert_group_setting_id: impl Into<AlertGroupSettingId>,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/alert-group-settings/{}", alert_group_setting_id),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::alert_group_setting::*;
    use crate::tests::*;

    fn value_example() -> AlertGroupSettingValue {
        AlertGroupSettingValue::builder()
            .name("Example alert group setting")
            .memo("This is an alert group setting memo.")
            .service_scopes(["service0".into()])
            .role_scopes(["service1:role1".into()])
            .monitor_scopes(["monitor0".into()])
            .notification_interval(60)
            .build()
    }

    fn entity_example() -> AlertGroupSetting {
        AlertGroupSetting {
            id: AlertGroupSettingId::from("setting0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "name": "Example alert group setting",
            "memo": "This is an alert group setting memo.",
            "serviceScopes": ["service0"],
            "roleScopes": ["service1:role1"],
            "monitorScopes": ["monitor0"],
            "notificationInterval": 60,
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("setting0");
        json
    }

    #[async_std::test]
    async fn list_alert_group_settings() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/alert-group-settings",
            response = json!({
                "alertGroupSettings": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_alert_group_settings().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn create_alert_group_setting() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/alert-group-settings",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .create_alert_group_setting(&value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn get_alert_group_setting() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/alert-group-settings/setting0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .get_alert_group_setting("setting0")
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .get_alert_group_setting(AlertGroupSettingId::from("setting0"))
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn update_alert_group_setting() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/alert-group-settings/setting0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_alert_group_setting("setting0", &value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_alert_group_setting(AlertGroupSettingId::from("setting0"), &value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn delete_alert_group_setting() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/alert-group-settings/setting0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .delete_alert_group_setting("setting0")
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_alert_group_setting(AlertGroupSettingId::from("setting0"))
                .await,
            Ok(entity_example()),
        );
    }
}
