use crate::channel::ChannelId;
use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::monitor::MonitorId;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

/// A notification group
pub type NotificationGroup = Entity<NotificationGroupValue>;

/// A notification group id
pub type NotificationGroupId = Id<NotificationGroupValue>;

/// A notification group id value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationGroupValue {
    pub name: String,
    pub notification_level: NotificationLevel,
    pub child_notification_group_ids: Vec<NotificationGroupId>,
    pub child_channel_ids: Vec<ChannelId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitors: Vec<NotificationGroupMonitor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<NotificationGroupService>,
}

/// A notification level
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum NotificationLevel {
    All,
    Critical,
}

/// A notification group monitor
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationGroupMonitor {
    pub id: MonitorId,
    pub skip_default: bool,
}

/// A notification group service
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationGroupService {
    pub name: ServiceName,
}

#[cfg(test)]
mod tests {
    use crate::notification_group::*;
    use serde_json::json;

    fn notification_group_example1() -> NotificationGroup {
        NotificationGroup {
            id: "abcde1".into(),
            value: NotificationGroupValue {
                name: "Example notification group".to_string(),
                notification_level: NotificationLevel::All,
                child_notification_group_ids: vec![],
                child_channel_ids: vec![],
                monitors: vec![],
                services: vec![],
            },
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "name": "Example notification group",
            "notificationLevel": "all",
            "childNotificationGroupIds": [],
            "childChannelIds": [],
        })
    }

    fn notification_group_example2() -> NotificationGroup {
        NotificationGroup {
            id: "abcde2".into(),
            value: NotificationGroupValue {
                name: "Example notification group".to_string(),
                notification_level: NotificationLevel::Critical,
                child_notification_group_ids: vec!["abcde3".into()],
                child_channel_ids: vec!["abcde4".into()],
                monitors: vec![NotificationGroupMonitor {
                    id: "abcde5".into(),
                    skip_default: false,
                }],
                services: vec![NotificationGroupService {
                    name: "ExampleService".into(),
                }],
            },
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "name": "Example notification group",
            "notificationLevel": "critical",
            "childNotificationGroupIds": ["abcde3"],
            "childChannelIds": ["abcde4"],
            "monitors": [{"id": "abcde5", "skipDefault": false}],
            "services": [{"name": "ExampleService"}],
        })
    }

    #[test]
    fn serialize_notification_group() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&notification_group_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&notification_group_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_notification_group() {
        assert_eq!(
            notification_group_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            notification_group_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListNotificationGroupsResponse {
    notification_groups: Vec<NotificationGroup>,
}

impl client::Client {
    /// Fetches all the notification groups.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#get>.
    pub async fn list_notification_groups(&self) -> Result<Vec<NotificationGroup>> {
        self.request(
            Method::GET,
            "/api/v0/notification-groups",
            vec![],
            client::empty_body(),
            |res: ListNotificationGroupsResponse| res.notification_groups,
        )
        .await
    }

    /// Creates a new notification group.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#create>.
    pub async fn create_notification_group(
        &self,
        notification_group_value: NotificationGroupValue,
    ) -> Result<NotificationGroup> {
        self.request(
            Method::POST,
            "/api/v0/notification-groups",
            vec![],
            Some(notification_group_value),
            |notification_group| notification_group,
        )
        .await
    }

    /// Updates a notification group.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#update>.
    pub async fn update_notification_group(
        &self,
        notification_group_id: NotificationGroupId,
        notification_group_value: NotificationGroupValue,
    ) -> Result<NotificationGroup> {
        self.request(
            Method::PUT,
            format!("/api/v0/notification-groups/{}", notification_group_id),
            vec![],
            Some(notification_group_value),
            |notification_group| notification_group,
        )
        .await
    }

    /// Deletes a notification group.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#delete>.
    pub async fn delete_notification_group(
        &self,
        notification_group_id: NotificationGroupId,
    ) -> Result<NotificationGroup> {
        self.request(
            Method::DELETE,
            format!("/api/v0/notification-groups/{}", notification_group_id),
            vec![],
            client::empty_body(),
            |notification_group| notification_group,
        )
        .await
    }
}
