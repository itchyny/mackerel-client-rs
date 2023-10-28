use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::monitor::MonitorId;
use crate::role::RoleFullname;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// An alert group setting
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
    use crate::alert_group_setting::*;
    use rstest::rstest;
    use serde_json::json;

    fn alert_group_setting_example1() -> AlertGroupSetting {
        AlertGroupSetting::builder()
            .id("abcde1")
            .value(
                AlertGroupSettingValue::builder()
                    .name("Example alert group setting")
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "name": "Example alert group setting",
        })
    }

    fn alert_group_setting_example2() -> AlertGroupSetting {
        AlertGroupSetting::builder()
            .id("abcde2")
            .value(
                AlertGroupSettingValue::builder()
                    .name("Example alert group setting")
                    .memo("This is an alert group setting memo.")
                    .service_scopes(["ExampleService".into()])
                    .role_scopes(["ExampleService:ExampleRole".into()])
                    .monitor_scopes(["monitor0".into()])
                    .notification_interval(60)
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "name": "Example alert group setting",
            "memo": "This is an alert group setting memo.",
            "serviceScopes": ["ExampleService"],
            "roleScopes": ["ExampleService:ExampleRole"],
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListAlertGroupSettingsResponse {
    alert_group_settings: Vec<AlertGroupSetting>,
}

impl client::Client {
    /// Fetches all the alert group settings.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#list>.
    pub async fn list_alert_group_settings(&self) -> Result<Vec<AlertGroupSetting>> {
        self.request(
            Method::GET,
            "/api/v0/alert-group-settings",
            vec![],
            client::empty_body(),
            |res: ListAlertGroupSettingsResponse| res.alert_group_settings,
        )
        .await
    }

    /// Creates a new alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#create>.
    pub async fn create_alert_group_setting(
        &self,
        alert_group_setting_value: AlertGroupSettingValue,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::POST,
            "/api/v0/alert-group-settings",
            vec![],
            Some(alert_group_setting_value),
            |alert_group_setting| alert_group_setting,
        )
        .await
    }

    /// Gets an alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#get>.
    pub async fn get_alert_group_setting(
        &self,
        alert_group_setting_id: AlertGroupSettingId,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::GET,
            format!("/api/v0/alert-group-settings/{}", alert_group_setting_id),
            vec![],
            client::empty_body(),
            |alert_group_setting| alert_group_setting,
        )
        .await
    }

    /// Updates an alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#update>.
    pub async fn update_alert_group_setting(
        &self,
        alert_group_setting_id: AlertGroupSettingId,
        alert_group_setting_value: AlertGroupSettingValue,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::PUT,
            format!("/api/v0/alert-group-settings/{}", alert_group_setting_id),
            vec![],
            Some(alert_group_setting_value),
            |alert_group_setting| alert_group_setting,
        )
        .await
    }

    /// Deletes an alert group setting.
    ///
    /// See <https://mackerel.io/api-docs/entry/alert-group-settings#delete>.
    pub async fn delete_alert_group_setting(
        &self,
        alert_group_setting_id: AlertGroupSettingId,
    ) -> Result<AlertGroupSetting> {
        self.request(
            Method::DELETE,
            format!("/api/v0/alert-group-settings/{}", alert_group_setting_id),
            vec![],
            client::empty_body(),
            |alert_group_setting| alert_group_setting,
        )
        .await
    }
}
