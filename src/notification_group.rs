use crate::channel::ChannelId;
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::monitor::MonitorId;
use crate::service::ServiceName;

/// A notification group entity
pub type NotificationGroup = Entity<NotificationGroupValue>;

/// A notification group id
pub type NotificationGroupId = Id<NotificationGroupValue>;

/// A notification group value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct NotificationGroupValue {
    pub name: String,
    #[builder(default)]
    pub notification_level: NotificationLevel,
    #[builder(default)]
    #[builder(
        default,
        setter(transform = |notification_group_ids: impl IntoIterator<Item = impl Into<NotificationGroupId>>| notification_group_ids
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    pub child_notification_group_ids: Vec<NotificationGroupId>,
    #[builder(
        default,
        setter(transform = |channel_ids: impl IntoIterator<Item = impl Into<ChannelId>>| channel_ids
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    pub child_channel_ids: Vec<ChannelId>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitors: Vec<NotificationGroupMonitor>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<NotificationGroupService>,
}

/// A notification level
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
pub enum NotificationLevel {
    #[default]
    All,
    Critical,
}

/// A notification group monitor
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct NotificationGroupMonitor {
    pub id: MonitorId,
    #[builder(default)]
    pub skip_default: bool,
}

/// A notification group service
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct NotificationGroupService {
    pub name: ServiceName,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn notification_group_example1() -> NotificationGroup {
        NotificationGroup::builder()
            .id("group1")
            .value(
                NotificationGroupValue::builder()
                    .name("Example notification group")
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "group1",
            "name": "Example notification group",
            "notificationLevel": "all",
            "childNotificationGroupIds": [],
            "childChannelIds": [],
        })
    }

    fn notification_group_example2() -> NotificationGroup {
        NotificationGroup::builder()
            .id("group2")
            .value(
                NotificationGroupValue::builder()
                    .name("Example notification group")
                    .notification_level(NotificationLevel::Critical)
                    .child_notification_group_ids(["group3"])
                    .child_channel_ids(["channel0"])
                    .monitors([NotificationGroupMonitor::builder().id("monitor0").build()])
                    .services([NotificationGroupService::builder().name("service0").build()])
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "group2",
            "name": "Example notification group",
            "notificationLevel": "critical",
            "childNotificationGroupIds": ["group3"],
            "childChannelIds": ["channel0"],
            "monitors": [{"id": "monitor0", "skipDefault": false}],
            "services": [{"name": "service0"}],
        })
    }

    #[rstest]
    #[case(notification_group_example1(), json_example1())]
    #[case(notification_group_example2(), json_example2())]
    fn test_notification_group(
        #[case] notification_group: NotificationGroup,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&notification_group).unwrap(), json);
        assert_eq!(notification_group, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Fetches all the notification groups.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#get>.
    pub async fn list_notification_groups(&self) -> Result<Vec<NotificationGroup>> {
        self.request(
            Method::GET,
            "/api/v0/notification-groups",
            query_params![],
            request_body![],
            response_body! { notificationGroups: Vec<NotificationGroup> },
        )
        .await
    }

    /// Creates a new notification group.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#create>.
    pub async fn create_notification_group(
        &self,
        notification_group_value: impl Borrow<NotificationGroupValue>,
    ) -> Result<NotificationGroup> {
        self.request(
            Method::POST,
            "/api/v0/notification-groups",
            query_params![],
            request_body!(notification_group_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Updates a notification group.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#update>.
    pub async fn update_notification_group(
        &self,
        notification_group_id: impl Into<NotificationGroupId>,
        notification_group_value: impl Borrow<NotificationGroupValue>,
    ) -> Result<NotificationGroup> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/notification-groups/{}", notification_group_id),
            query_params![],
            request_body!(notification_group_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Deletes a notification group.
    ///
    /// See <https://mackerel.io/api-docs/entry/notification-groups#delete>.
    pub async fn delete_notification_group(
        &self,
        notification_group_id: impl Into<NotificationGroupId>,
    ) -> Result<NotificationGroup> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/notification-groups/{}", notification_group_id),
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

    use crate::notification_group::*;
    use crate::tests::*;

    fn value_example() -> NotificationGroupValue {
        NotificationGroupValue::builder()
            .name("Example notification group")
            .notification_level(NotificationLevel::Critical)
            .child_notification_group_ids(["group1"])
            .child_channel_ids(["channel0"])
            .monitors([NotificationGroupMonitor::builder()
                .id("monitor0")
                .skip_default(true)
                .build()])
            .services([NotificationGroupService::builder().name("service0").build()])
            .build()
    }

    fn entity_example() -> NotificationGroup {
        NotificationGroup {
            id: NotificationGroupId::from("group0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "name": "Example notification group",
            "notificationLevel": "critical",
            "childNotificationGroupIds": ["group1"],
            "childChannelIds": ["channel0"],
            "monitors": [{"id": "monitor0", "skipDefault": true}],
            "services": [{"name": "service0"}],
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("group0");
        json
    }

    #[async_std::test]
    async fn list_notification_groups() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/notification-groups",
            response = json!({
                "notificationGroups": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_notification_groups().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn create_notification_group() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/notification-groups",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .create_notification_group(value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .create_notification_group(&value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn update_notification_group() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/notification-groups/group0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_notification_group("group0", value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_notification_group(NotificationGroupId::from("group0"), &value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn delete_notification_group() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/notification-groups/group0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .delete_notification_group("group0")
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_notification_group(NotificationGroupId::from("group0"))
                .await,
            Ok(entity_example()),
        );
    }
}
