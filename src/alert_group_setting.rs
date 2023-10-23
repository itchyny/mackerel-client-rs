use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::monitor::MonitorId;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// An alert group setting
pub type AlertGroupSetting = Entity<AlertGroupSettingValue>;

/// An alert group setting id
pub type AlertGroupSettingId = Id<AlertGroupSettingValue>;

/// An alert group setting value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertGroupSettingValue {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_scopes: Vec<MonitorId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_interval: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::alert_group_setting::*;
    use serde_json::json;

    fn alert_group_setting_example1() -> AlertGroupSetting {
        AlertGroupSetting {
            id: "abcde1".into(),
            value: AlertGroupSettingValue {
                name: "Example alert group setting".to_string(),
                memo: "This is an alert group setting memo.".to_string(),
                service_scopes: vec![],
                role_scopes: vec![],
                monitor_scopes: vec![],
                notification_interval: None,
            },
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "name": "Example alert group setting",
            "memo": "This is an alert group setting memo.",
        })
    }

    fn alert_group_setting_example2() -> AlertGroupSetting {
        AlertGroupSetting {
            id: "abcde2".into(),
            value: AlertGroupSettingValue {
                name: "Example alert group setting".to_string(),
                memo: "".to_string(),
                service_scopes: vec!["ExampleService".to_string()],
                role_scopes: vec!["ExampleService:ExampleRole".to_string()],
                monitor_scopes: vec!["monitor0".into()],
                notification_interval: Some(60),
            },
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "name": "Example alert group setting",
            "serviceScopes": ["ExampleService"],
            "roleScopes": ["ExampleService:ExampleRole"],
            "monitorScopes": ["monitor0"],
            "notificationInterval": 60,
        })
    }

    #[test]
    fn serialize_alert_group_setting() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&alert_group_setting_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&alert_group_setting_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_alert_group_setting() {
        assert_eq!(
            alert_group_setting_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            alert_group_setting_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
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
