use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use strum::{Display, EnumString};

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::user::UserId;

/// A channel entity
pub type Channel = Entity<ChannelValue>;

/// A channel id
pub type ChannelId = Id<ChannelValue>;

/// A channel value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ChannelValue {
    #[serde(rename_all = "camelCase")]
    Email {
        name: String,
        #[serde(default)]
        emails: Vec<String>,
        #[serde(default)]
        user_ids: Vec<UserId>,
        #[serde(default)]
        events: Vec<NotificationEvent>,
    },
    #[serde(rename_all = "camelCase")]
    Slack {
        name: String,
        url: String,
        enabled_graph_image: bool,
        mentions: HashMap<String, String>,
        #[serde(default)]
        events: Vec<NotificationEvent>,
    },
    Line {
        name: String,
    },
    Chatwork {
        name: String,
    },
    Typetalk {
        name: String,
    },
    Twilio {
        name: String,
    },
    Pagerduty {
        name: String,
    },
    Opsgenie {
        name: String,
    },
    Yammer {
        name: String,
    },
    MicrosoftTeams {
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    Webhook {
        name: String,
        url: String,
        enabled_graph_image: bool,
        #[serde(default)]
        events: Vec<NotificationEvent>,
    },
    AmazonEventBridge {
        name: String,
    },
}

impl ChannelValue {
    /// Returns the name of the channel.
    pub fn name(&self) -> String {
        match *self {
            Self::Email { ref name, .. } => name.clone(),
            Self::Slack { ref name, .. } => name.clone(),
            Self::Line { ref name, .. } => name.clone(),
            Self::Chatwork { ref name, .. } => name.clone(),
            Self::Typetalk { ref name, .. } => name.clone(),
            Self::Twilio { ref name, .. } => name.clone(),
            Self::Pagerduty { ref name, .. } => name.clone(),
            Self::Opsgenie { ref name, .. } => name.clone(),
            Self::Yammer { ref name, .. } => name.clone(),
            Self::MicrosoftTeams { ref name, .. } => name.clone(),
            Self::Webhook { ref name, .. } => name.clone(),
            Self::AmazonEventBridge { ref name, .. } => name.clone(),
        }
    }
}

/// Notification event
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "camelCase")]
pub enum NotificationEvent {
    Alert,
    AlertGroup,
    HostStatus,
    HostRegister,
    HostRetire,
    Monitor,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn email_channel_example() -> Channel {
        Channel::builder()
            .id("channel1")
            .value(ChannelValue::Email {
                name: "Example Email Channel".to_string(),
                emails: vec!["test@example.com".to_string()],
                user_ids: vec!["user0".into()],
                events: vec![NotificationEvent::Alert],
            })
            .build()
    }

    fn email_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel1",
            "type": "email",
            "name": "Example Email Channel",
            "emails": ["test@example.com"],
            "userIds": ["user0"],
            "events": ["alert"],
        })
    }

    fn slack_channel_example() -> Channel {
        Channel::builder()
            .id("channel2")
            .value(ChannelValue::Slack {
                name: "Example Slack Channel".to_string(),
                url: "slack@example.com".to_string(),
                enabled_graph_image: true,
                mentions: HashMap::from([("critical".to_string(), "@channel".to_string())]),
                events: vec![
                    NotificationEvent::HostStatus,
                    NotificationEvent::HostRegister,
                    NotificationEvent::HostRetire,
                ],
            })
            .build()
    }

    fn slack_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel2",
            "type": "slack",
            "name": "Example Slack Channel",
            "url": "slack@example.com",
            "enabledGraphImage": true,
            "mentions": {"critical": "@channel"},
            "events": ["hostStatus", "hostRegister", "hostRetire"],
        })
    }

    fn line_channel_example() -> Channel {
        Channel::builder()
            .id("channel3")
            .value(ChannelValue::Line {
                name: "Example Line Channel".to_string(),
            })
            .build()
    }

    fn line_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel3",
            "type": "line",
            "name": "Example Line Channel",
        })
    }

    fn chatwork_channel_example() -> Channel {
        Channel::builder()
            .id("channel4")
            .value(ChannelValue::Chatwork {
                name: "Example Chatwork Channel".to_string(),
            })
            .build()
    }

    fn chatwork_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel4",
            "type": "chatwork",
            "name": "Example Chatwork Channel",
        })
    }

    fn typetalk_channel_example() -> Channel {
        Channel::builder()
            .id("channel5")
            .value(ChannelValue::Typetalk {
                name: "Example Typetalk Channel".to_string(),
            })
            .build()
    }

    fn typetalk_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel5",
            "type": "typetalk",
            "name": "Example Typetalk Channel",
        })
    }

    fn twilio_channel_example() -> Channel {
        Channel::builder()
            .id("channel6")
            .value(ChannelValue::Twilio {
                name: "Example Twilio Channel".to_string(),
            })
            .build()
    }

    fn twilio_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel6",
            "type": "twilio",
            "name": "Example Twilio Channel",
        })
    }

    fn pagerduty_channel_example() -> Channel {
        Channel::builder()
            .id("channel7")
            .value(ChannelValue::Pagerduty {
                name: "Example Pagerduty Channel".to_string(),
            })
            .build()
    }

    fn pagerduty_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel7",
            "type": "pagerduty",
            "name": "Example Pagerduty Channel",
        })
    }

    fn opsgenie_channel_example() -> Channel {
        Channel::builder()
            .id("channel8")
            .value(ChannelValue::Opsgenie {
                name: "Example Opsgenie Channel".to_string(),
            })
            .build()
    }

    fn opsgenie_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel8",
            "type": "opsgenie",
            "name": "Example Opsgenie Channel",
        })
    }

    fn yammer_channel_example() -> Channel {
        Channel::builder()
            .id("channel9")
            .value(ChannelValue::Yammer {
                name: "Example Yammer Channel".to_string(),
            })
            .build()
    }

    fn yammer_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel9",
            "type": "yammer",
            "name": "Example Yammer Channel",
        })
    }

    fn microsoft_teams_channel_example() -> Channel {
        Channel::builder()
            .id("channel10")
            .value(ChannelValue::MicrosoftTeams {
                name: "Example MicrosoftTeams Channel".to_string(),
            })
            .build()
    }

    fn microsoft_teams_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel10",
            "type": "microsoft-teams",
            "name": "Example MicrosoftTeams Channel",
        })
    }

    fn webhook_channel_example() -> Channel {
        Channel::builder()
            .id("channel11")
            .value(ChannelValue::Webhook {
                name: "Example Webhook Channel".to_string(),
                url: "webhook@example.com".to_string(),
                enabled_graph_image: true,
                events: vec![
                    NotificationEvent::Alert,
                    NotificationEvent::AlertGroup,
                    NotificationEvent::Monitor,
                ],
            })
            .build()
    }

    fn webhook_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel11",
            "type": "webhook",
            "name": "Example Webhook Channel",
            "url": "webhook@example.com",
            "enabledGraphImage": true,
            "events": ["alert", "alertGroup", "monitor"],
        })
    }

    fn amazon_event_bridge_channel_example() -> Channel {
        Channel::builder()
            .id("channel12")
            .value(ChannelValue::AmazonEventBridge {
                name: "Example AmazonEventBridge Channel".to_string(),
            })
            .build()
    }

    fn amazon_event_bridge_json_example() -> serde_json::Value {
        json!({
            "id": "channel12",
            "type": "amazon-event-bridge",
            "name": "Example AmazonEventBridge Channel",
        })
    }

    #[rstest]
    #[case(email_channel_example(), email_channel_json_example())]
    #[case(slack_channel_example(), slack_channel_json_example())]
    #[case(line_channel_example(), line_channel_json_example())]
    #[case(chatwork_channel_example(), chatwork_channel_json_example())]
    #[case(typetalk_channel_example(), typetalk_channel_json_example())]
    #[case(twilio_channel_example(), twilio_channel_json_example())]
    #[case(pagerduty_channel_example(), pagerduty_channel_json_example())]
    #[case(opsgenie_channel_example(), opsgenie_channel_json_example())]
    #[case(yammer_channel_example(), yammer_channel_json_example())]
    #[case(
        microsoft_teams_channel_example(),
        microsoft_teams_channel_json_example()
    )]
    #[case(webhook_channel_example(), webhook_channel_json_example())]
    #[case(
        amazon_event_bridge_channel_example(),
        amazon_event_bridge_json_example()
    )]
    fn test_channel_json(#[case] channel: Channel, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&channel).unwrap(), json);
        assert_eq!(channel, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(email_channel_example(), "Example Email Channel")]
    #[case(slack_channel_example(), "Example Slack Channel")]
    #[case(line_channel_example(), "Example Line Channel")]
    #[case(chatwork_channel_example(), "Example Chatwork Channel")]
    #[case(typetalk_channel_example(), "Example Typetalk Channel")]
    #[case(twilio_channel_example(), "Example Twilio Channel")]
    #[case(pagerduty_channel_example(), "Example Pagerduty Channel")]
    #[case(opsgenie_channel_example(), "Example Opsgenie Channel")]
    #[case(yammer_channel_example(), "Example Yammer Channel")]
    #[case(microsoft_teams_channel_example(), "Example MicrosoftTeams Channel")]
    #[case(webhook_channel_example(), "Example Webhook Channel")]
    #[case(
        amazon_event_bridge_channel_example(),
        "Example AmazonEventBridge Channel"
    )]
    fn test_channel_name(#[case] channel: Channel, #[case] name_str: &str) {
        assert_eq!(channel.name(), name_str);
    }

    #[rstest]
    #[case(NotificationEvent::Alert, "alert")]
    #[case(NotificationEvent::AlertGroup, "alertGroup")]
    #[case(NotificationEvent::HostStatus, "hostStatus")]
    #[case(NotificationEvent::HostRegister, "hostRegister")]
    #[case(NotificationEvent::HostRetire, "hostRetire")]
    #[case(NotificationEvent::Monitor, "monitor")]
    fn test_notification_event(
        #[case] notification_event: NotificationEvent,
        #[case] notification_event_str: &str,
    ) {
        assert_eq!(notification_event.to_string(), notification_event_str);
        assert_eq!(notification_event, notification_event_str.parse().unwrap());
        assert_eq!(
            notification_event,
            serde_json::from_value(notification_event_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(notification_event).unwrap(),
            notification_event_str
        );
    }
}

impl Client {
    /// Fetches all the channels.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#get>.
    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        self.request(
            Method::GET,
            "/api/v0/channels",
            query_params![],
            request_body![],
            response_body! { channels: Vec<Channel> },
        )
        .await
    }

    /// Creates a new channel.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#create>.
    pub async fn create_channel(&self, channel_value: &ChannelValue) -> Result<Channel> {
        self.request(
            Method::POST,
            "/api/v0/channels",
            query_params![],
            request_body!(channel_value),
            response_body!(..),
        )
        .await
    }

    /// Deletes a channel.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#delete>.
    pub async fn delete_channel(&self, channel_id: impl Into<ChannelId>) -> Result<Channel> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/channels/{}", channel_id),
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

    use crate::channel::*;
    use crate::tests::*;

    fn value_example() -> ChannelValue {
        ChannelValue::Email {
            name: "Example Email Channel".to_string(),
            emails: vec!["mackerel@example.com".to_string()],
            user_ids: vec!["user0".into()],
            events: vec![NotificationEvent::Alert],
        }
    }

    fn entity_example() -> Channel {
        Channel {
            id: ChannelId::from("channel0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "type": "email",
            "name": "Example Email Channel",
            "emails": ["mackerel@example.com"],
            "userIds": ["user0"],
            "events": ["alert"],
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("channel0");
        json
    }

    #[async_std::test]
    async fn list_channels() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/channels",
            response = json!({
                "channels": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_channels().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn create_channel() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/channels",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).create_channel(&value_example()).await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn delete_channel() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/channels/channel0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).delete_channel("channel0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_channel(ChannelId::from("channel0"))
                .await,
            Ok(entity_example()),
        );
    }
}
