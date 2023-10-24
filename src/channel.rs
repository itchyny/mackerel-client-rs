use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::user::UserId;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use strum::{Display, EnumString};

/// A channel
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
    pub fn get_name(&self) -> String {
        match *self {
            ChannelValue::Email { ref name, .. } => name.clone(),
            ChannelValue::Slack { ref name, .. } => name.clone(),
            ChannelValue::Line { ref name, .. } => name.clone(),
            ChannelValue::Chatwork { ref name, .. } => name.clone(),
            ChannelValue::Typetalk { ref name, .. } => name.clone(),
            ChannelValue::Twilio { ref name, .. } => name.clone(),
            ChannelValue::Pagerduty { ref name, .. } => name.clone(),
            ChannelValue::Opsgenie { ref name, .. } => name.clone(),
            ChannelValue::Yammer { ref name, .. } => name.clone(),
            ChannelValue::MicrosoftTeams { ref name, .. } => name.clone(),
            ChannelValue::Webhook { ref name, .. } => name.clone(),
            ChannelValue::AmazonEventBridge { ref name, .. } => name.clone(),
        }
    }
}

/// Notification events
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
    use crate::channel::*;
    use serde_json::json;

    fn email_channel_example() -> Channel {
        Channel {
            id: "abcde1".into(),
            value: ChannelValue::Email {
                name: "Example Email Channel".to_string(),
                emails: vec!["test@example.com".to_string()],
                user_ids: vec!["abcde2".into()],
                events: vec![NotificationEvent::Alert],
            },
        }
    }

    fn email_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "type": "email",
            "name": "Example Email Channel",
            "emails": ["test@example.com"],
            "userIds": ["abcde2"],
            "events": ["alert"],
        })
    }

    fn slack_channel_example() -> Channel {
        Channel {
            id: "abcde2".into(),
            value: ChannelValue::Slack {
                name: "Example Slack Channel".to_string(),
                url: "slack@example.com".to_string(),
                enabled_graph_image: true,
                mentions: HashMap::from([("critical".to_string(), "@channel".to_string())]),
                events: vec![
                    NotificationEvent::HostStatus,
                    NotificationEvent::HostRegister,
                    NotificationEvent::HostRetire,
                ],
            },
        }
    }

    fn slack_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "type": "slack",
            "name": "Example Slack Channel",
            "url": "slack@example.com",
            "enabledGraphImage": true,
            "mentions": {"critical": "@channel"},
            "events": ["hostStatus", "hostRegister", "hostRetire"],
        })
    }

    fn line_channel_example() -> Channel {
        Channel {
            id: "abcde3".into(),
            value: ChannelValue::Line {
                name: "Example Line Channel".to_string(),
            },
        }
    }

    fn line_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde3",
            "type": "line",
            "name": "Example Line Channel",
        })
    }

    fn chatwork_channel_example() -> Channel {
        Channel {
            id: "abcde4".into(),
            value: ChannelValue::Chatwork {
                name: "Example Chatwork Channel".to_string(),
            },
        }
    }

    fn chatwork_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde4",
            "type": "chatwork",
            "name": "Example Chatwork Channel",
        })
    }

    fn typetalk_channel_example() -> Channel {
        Channel {
            id: "abcde5".into(),
            value: ChannelValue::Typetalk {
                name: "Example Typetalk Channel".to_string(),
            },
        }
    }

    fn typetalk_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde5",
            "type": "typetalk",
            "name": "Example Typetalk Channel",
        })
    }

    fn twilio_channel_example() -> Channel {
        Channel {
            id: "abcde6".into(),
            value: ChannelValue::Twilio {
                name: "Example Twilio Channel".to_string(),
            },
        }
    }

    fn twilio_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde6",
            "type": "twilio",
            "name": "Example Twilio Channel",
        })
    }

    fn pagerduty_channel_example() -> Channel {
        Channel {
            id: "abcde7".into(),
            value: ChannelValue::Pagerduty {
                name: "Example Pagerduty Channel".to_string(),
            },
        }
    }

    fn pagerduty_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde7",
            "type": "pagerduty",
            "name": "Example Pagerduty Channel",
        })
    }

    fn opsgenie_channel_example() -> Channel {
        Channel {
            id: "abcde8".into(),
            value: ChannelValue::Opsgenie {
                name: "Example Opsgenie Channel".to_string(),
            },
        }
    }

    fn opsgenie_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde8",
            "type": "opsgenie",
            "name": "Example Opsgenie Channel",
        })
    }

    fn yammer_channel_example() -> Channel {
        Channel {
            id: "abcde9".into(),
            value: ChannelValue::Yammer {
                name: "Example Yammer Channel".to_string(),
            },
        }
    }

    fn yammer_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcde9",
            "type": "yammer",
            "name": "Example Yammer Channel",
        })
    }

    fn microsoft_teams_channel_example() -> Channel {
        Channel {
            id: "abcdea".into(),
            value: ChannelValue::MicrosoftTeams {
                name: "Example MicrosoftTeams Channel".to_string(),
            },
        }
    }

    fn microsoft_teams_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcdea",
            "type": "microsoft-teams",
            "name": "Example MicrosoftTeams Channel",
        })
    }

    fn webhook_channel_example() -> Channel {
        Channel {
            id: "abcdeb".into(),
            value: ChannelValue::Webhook {
                name: "Example Webhook Channel".to_string(),
                url: "webhook@example.com".to_string(),
                enabled_graph_image: true,
                events: vec![
                    NotificationEvent::Alert,
                    NotificationEvent::AlertGroup,
                    NotificationEvent::Monitor,
                ],
            },
        }
    }

    fn webhook_channel_json_example() -> serde_json::Value {
        json!({
            "id": "abcdeb",
            "type": "webhook",
            "name": "Example Webhook Channel",
            "url": "webhook@example.com",
            "enabledGraphImage": true,
            "events": ["alert", "alertGroup", "monitor"],
        })
    }

    fn amazon_event_bridge_channel_example() -> Channel {
        Channel {
            id: "abcdec".into(),
            value: ChannelValue::AmazonEventBridge {
                name: "Example AmazonEventBridge Channel".to_string(),
            },
        }
    }

    fn amazon_event_bridge_json_example() -> serde_json::Value {
        json!({
            "id": "abcdec",
            "type": "amazon-event-bridge",
            "name": "Example AmazonEventBridge Channel",
        })
    }

    #[test]
    fn channel_name() {
        assert_eq!(
            email_channel_example().get_name(),
            "Example Email Channel".to_string(),
        );
        assert_eq!(
            slack_channel_example().get_name(),
            "Example Slack Channel".to_string(),
        );
        assert_eq!(
            line_channel_example().get_name(),
            "Example Line Channel".to_string(),
        );
        assert_eq!(
            chatwork_channel_example().get_name(),
            "Example Chatwork Channel".to_string(),
        );
        assert_eq!(
            typetalk_channel_example().get_name(),
            "Example Typetalk Channel".to_string(),
        );
        assert_eq!(
            twilio_channel_example().get_name(),
            "Example Twilio Channel".to_string(),
        );
        assert_eq!(
            pagerduty_channel_example().get_name(),
            "Example Pagerduty Channel".to_string(),
        );
        assert_eq!(
            opsgenie_channel_example().get_name(),
            "Example Opsgenie Channel".to_string(),
        );
        assert_eq!(
            yammer_channel_example().get_name(),
            "Example Yammer Channel".to_string(),
        );
        assert_eq!(
            microsoft_teams_channel_example().get_name(),
            "Example MicrosoftTeams Channel".to_string(),
        );
        assert_eq!(
            webhook_channel_example().get_name(),
            "Example Webhook Channel".to_string(),
        );
        assert_eq!(
            amazon_event_bridge_channel_example().get_name(),
            "Example AmazonEventBridge Channel".to_string(),
        );
    }

    #[test]
    fn serialize_channel() {
        assert_eq!(
            email_channel_json_example(),
            serde_json::to_value(&email_channel_example()).unwrap()
        );
        assert_eq!(
            slack_channel_json_example(),
            serde_json::to_value(&slack_channel_example()).unwrap()
        );
        assert_eq!(
            line_channel_json_example(),
            serde_json::to_value(&line_channel_example()).unwrap()
        );
        assert_eq!(
            chatwork_channel_json_example(),
            serde_json::to_value(&chatwork_channel_example()).unwrap()
        );
        assert_eq!(
            typetalk_channel_json_example(),
            serde_json::to_value(&typetalk_channel_example()).unwrap()
        );
        assert_eq!(
            twilio_channel_json_example(),
            serde_json::to_value(&twilio_channel_example()).unwrap()
        );
        assert_eq!(
            pagerduty_channel_json_example(),
            serde_json::to_value(&pagerduty_channel_example()).unwrap()
        );
        assert_eq!(
            opsgenie_channel_json_example(),
            serde_json::to_value(&opsgenie_channel_example()).unwrap()
        );
        assert_eq!(
            yammer_channel_json_example(),
            serde_json::to_value(&yammer_channel_example()).unwrap()
        );
        assert_eq!(
            microsoft_teams_channel_json_example(),
            serde_json::to_value(&microsoft_teams_channel_example()).unwrap()
        );
        assert_eq!(
            webhook_channel_json_example(),
            serde_json::to_value(&webhook_channel_example()).unwrap()
        );
        assert_eq!(
            amazon_event_bridge_json_example(),
            serde_json::to_value(&amazon_event_bridge_channel_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_channel() {
        assert_eq!(
            email_channel_example(),
            serde_json::from_value(email_channel_json_example()).unwrap()
        );
        assert_eq!(
            slack_channel_example(),
            serde_json::from_value(slack_channel_json_example()).unwrap()
        );
        assert_eq!(
            line_channel_example(),
            serde_json::from_value(line_channel_json_example()).unwrap()
        );
        assert_eq!(
            chatwork_channel_example(),
            serde_json::from_value(chatwork_channel_json_example()).unwrap()
        );
        assert_eq!(
            typetalk_channel_example(),
            serde_json::from_value(typetalk_channel_json_example()).unwrap()
        );
        assert_eq!(
            twilio_channel_example(),
            serde_json::from_value(twilio_channel_json_example()).unwrap()
        );
        assert_eq!(
            pagerduty_channel_example(),
            serde_json::from_value(pagerduty_channel_json_example()).unwrap()
        );
        assert_eq!(
            opsgenie_channel_example(),
            serde_json::from_value(opsgenie_channel_json_example()).unwrap()
        );
        assert_eq!(
            yammer_channel_example(),
            serde_json::from_value(yammer_channel_json_example()).unwrap()
        );
        assert_eq!(
            microsoft_teams_channel_example(),
            serde_json::from_value(microsoft_teams_channel_json_example()).unwrap()
        );
        assert_eq!(
            webhook_channel_example(),
            serde_json::from_value(webhook_channel_json_example()).unwrap()
        );
        assert_eq!(
            amazon_event_bridge_channel_example(),
            serde_json::from_value(amazon_event_bridge_json_example()).unwrap()
        );
    }
}

#[derive(Deserialize)]
struct ListChannelResponse {
    channels: Vec<Channel>,
}

impl client::Client {
    /// Fetches all the channels.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#get>.
    pub async fn list_channels(&self) -> Result<Vec<Channel>> {
        self.request(
            Method::GET,
            "/api/v0/channels",
            vec![],
            client::empty_body(),
            |res: ListChannelResponse| res.channels,
        )
        .await
    }

    /// Creates a new channel.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#create>.
    pub async fn create_channel(&self, channel_value: ChannelValue) -> Result<Channel> {
        self.request(
            Method::POST,
            "/api/v0/channels",
            vec![],
            Some(channel_value),
            |channel| channel,
        )
        .await
    }

    /// Deletes a channel.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#delete>.
    pub async fn delete_channel(&self, channel_id: ChannelId) -> Result<Channel> {
        self.request(
            Method::DELETE,
            format!("/api/v0/channels/{}", channel_id),
            vec![],
            client::empty_body(),
            |channel| channel,
        )
        .await
    }
}
