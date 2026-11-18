use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use std::collections::HashMap;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

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
    Email(EmailChannel),
    Slack(SlackChannel),
    Line(LineChannel),
    Chatwork(ChatworkChannel),
    Typetalk(TypetalkChannel),
    Twilio(TwilioChannel),
    Pagerduty(PagerdutyChannel),
    Opsgenie(OpsgenieChannel),
    Yammer(YammerChannel),
    MicrosoftTeams(MicrosoftTeamsChannel),
    Webhook(WebhookChannel),
    AmazonEventBridge(AmazonEventBridgeChannel),
}

/// An email channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EmailChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
    #[builder(
        default,
        setter(transform = |emails: impl IntoIterator<Item = impl Into<String>>| emails
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default)]
    pub emails: Vec<String>,
    #[builder(
        default,
        setter(transform = |user_ids: impl IntoIterator<Item = impl Into<UserId>>| user_ids
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
    #[serde(default)]
    pub user_ids: Vec<UserId>,
    #[builder(
        default,
        setter(transform = |events: impl IntoIterator<Item = NotificationEvent>| events
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(default)]
    pub events: Vec<NotificationEvent>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_to_default_notification_group: Option<bool>,
}

/// A Slack channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SlackChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
    pub url: String,
    #[builder(default)]
    pub enabled_graph_image: bool,
    #[builder(default)]
    pub mentions: HashMap<String, String>,
    #[builder(
        default,
        setter(transform = |events: impl IntoIterator<Item = NotificationEvent>| events
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(default)]
    pub events: Vec<NotificationEvent>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_to_default_notification_group: Option<bool>,
}

/// A LINE channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LineChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A Chatwork channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ChatworkChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A Typetalk channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TypetalkChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A Twilio channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TwilioChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A PagerDuty channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PagerdutyChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// An Opsgenie channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OpsgenieChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A Yammer channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct YammerChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A Microsoft Teams channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MicrosoftTeamsChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

/// A webhook channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WebhookChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
    pub url: String,
    #[builder(default)]
    pub enabled_graph_image: bool,
    #[builder(
        default,
        setter(transform = |events: impl IntoIterator<Item = NotificationEvent>| events
            .into_iter().collect::<Vec<_>>()),
    )]
    #[serde(default)]
    pub events: Vec<NotificationEvent>,
    #[builder(default, setter(!into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_to_default_notification_group: Option<bool>,
}

/// An Amazon EventBridge channel
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AmazonEventBridgeChannel {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(
        default,
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_at: Option<DateTime<Utc>>,
}

impl From<EmailChannel> for ChannelValue {
    fn from(channel: EmailChannel) -> Self {
        Self::Email(channel)
    }
}

impl From<SlackChannel> for ChannelValue {
    fn from(channel: SlackChannel) -> Self {
        Self::Slack(channel)
    }
}

impl From<LineChannel> for ChannelValue {
    fn from(channel: LineChannel) -> Self {
        Self::Line(channel)
    }
}

impl From<ChatworkChannel> for ChannelValue {
    fn from(channel: ChatworkChannel) -> Self {
        Self::Chatwork(channel)
    }
}

impl From<TypetalkChannel> for ChannelValue {
    fn from(channel: TypetalkChannel) -> Self {
        Self::Typetalk(channel)
    }
}

impl From<TwilioChannel> for ChannelValue {
    fn from(channel: TwilioChannel) -> Self {
        Self::Twilio(channel)
    }
}

impl From<PagerdutyChannel> for ChannelValue {
    fn from(channel: PagerdutyChannel) -> Self {
        Self::Pagerduty(channel)
    }
}

impl From<OpsgenieChannel> for ChannelValue {
    fn from(channel: OpsgenieChannel) -> Self {
        Self::Opsgenie(channel)
    }
}

impl From<YammerChannel> for ChannelValue {
    fn from(channel: YammerChannel) -> Self {
        Self::Yammer(channel)
    }
}

impl From<MicrosoftTeamsChannel> for ChannelValue {
    fn from(channel: MicrosoftTeamsChannel) -> Self {
        Self::MicrosoftTeams(channel)
    }
}

impl From<WebhookChannel> for ChannelValue {
    fn from(channel: WebhookChannel) -> Self {
        Self::Webhook(channel)
    }
}

impl From<AmazonEventBridgeChannel> for ChannelValue {
    fn from(channel: AmazonEventBridgeChannel) -> Self {
        Self::AmazonEventBridge(channel)
    }
}

impl ChannelValue {
    /// Returns the name of the channel.
    pub fn name(&self) -> String {
        match self {
            Self::Email(channel) => channel.name.clone(),
            Self::Slack(channel) => channel.name.clone(),
            Self::Line(channel) => channel.name.clone(),
            Self::Chatwork(channel) => channel.name.clone(),
            Self::Typetalk(channel) => channel.name.clone(),
            Self::Twilio(channel) => channel.name.clone(),
            Self::Pagerduty(channel) => channel.name.clone(),
            Self::Opsgenie(channel) => channel.name.clone(),
            Self::Yammer(channel) => channel.name.clone(),
            Self::MicrosoftTeams(channel) => channel.name.clone(),
            Self::Webhook(channel) => channel.name.clone(),
            Self::AmazonEventBridge(channel) => channel.name.clone(),
        }
    }

    /// Returns the suspended_at of the channel.
    pub fn suspended_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Email(channel) => channel.suspended_at,
            Self::Slack(channel) => channel.suspended_at,
            Self::Line(channel) => channel.suspended_at,
            Self::Chatwork(channel) => channel.suspended_at,
            Self::Typetalk(channel) => channel.suspended_at,
            Self::Twilio(channel) => channel.suspended_at,
            Self::Pagerduty(channel) => channel.suspended_at,
            Self::Opsgenie(channel) => channel.suspended_at,
            Self::Yammer(channel) => channel.suspended_at,
            Self::MicrosoftTeams(channel) => channel.suspended_at,
            Self::Webhook(channel) => channel.suspended_at,
            Self::AmazonEventBridge(channel) => channel.suspended_at,
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
            .value(
                EmailChannel::builder()
                    .name("Example Email Channel")
                    .suspended_at(DateTime::from_timestamp(1711360000, 0).unwrap())
                    .emails(["test@example.com"])
                    .user_ids(["user0"])
                    .events([NotificationEvent::Alert])
                    .add_to_default_notification_group(true)
                    .build(),
            )
            .build()
    }

    fn email_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel1",
            "type": "email",
            "name": "Example Email Channel",
            "suspendedAt": 1711360000,
            "emails": ["test@example.com"],
            "userIds": ["user0"],
            "events": ["alert"],
            "addToDefaultNotificationGroup": true,
        })
    }

    fn slack_channel_example() -> Channel {
        Channel::builder()
            .id("channel2")
            .value(
                SlackChannel::builder()
                    .name("Example Slack Channel")
                    .suspended_at(DateTime::from_timestamp(1711360000, 0).unwrap())
                    .url("slack@example.com")
                    .enabled_graph_image(true)
                    .mentions(HashMap::from([(
                        "critical".to_string(),
                        "@channel".to_string(),
                    )]))
                    .events([
                        NotificationEvent::HostStatus,
                        NotificationEvent::HostRegister,
                        NotificationEvent::HostRetire,
                    ])
                    .build(),
            )
            .build()
    }

    fn slack_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel2",
            "type": "slack",
            "name": "Example Slack Channel",
            "suspendedAt": 1711360000,
            "url": "slack@example.com",
            "enabledGraphImage": true,
            "mentions": {"critical": "@channel"},
            "events": ["hostStatus", "hostRegister", "hostRetire"],
        })
    }

    fn line_channel_example() -> Channel {
        Channel::builder()
            .id("channel3")
            .value(
                LineChannel::builder()
                    .name("Example Line Channel")
                    .suspended_at(DateTime::from_timestamp(1711360000, 0).unwrap())
                    .build(),
            )
            .build()
    }

    fn line_channel_json_example() -> serde_json::Value {
        json!({
            "id": "channel3",
            "type": "line",
            "name": "Example Line Channel",
            "suspendedAt": 1711360000,
        })
    }

    fn chatwork_channel_example() -> Channel {
        Channel::builder()
            .id("channel4")
            .value(
                ChatworkChannel::builder()
                    .name("Example Chatwork Channel")
                    .build(),
            )
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
            .value(
                TypetalkChannel::builder()
                    .name("Example Typetalk Channel")
                    .build(),
            )
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
            .value(
                TwilioChannel::builder()
                    .name("Example Twilio Channel")
                    .build(),
            )
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
            .value(
                PagerdutyChannel::builder()
                    .name("Example Pagerduty Channel")
                    .build(),
            )
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
            .value(
                OpsgenieChannel::builder()
                    .name("Example Opsgenie Channel")
                    .build(),
            )
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
            .value(
                YammerChannel::builder()
                    .name("Example Yammer Channel")
                    .build(),
            )
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
            .value(
                MicrosoftTeamsChannel::builder()
                    .name("Example MicrosoftTeams Channel")
                    .build(),
            )
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
            .value(
                WebhookChannel::builder()
                    .name("Example Webhook Channel")
                    .url("webhook@example.com")
                    .enabled_graph_image(true)
                    .events([
                        NotificationEvent::Alert,
                        NotificationEvent::AlertGroup,
                        NotificationEvent::Monitor,
                    ])
                    .build(),
            )
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
            .value(
                AmazonEventBridgeChannel::builder()
                    .name("Example AmazonEventBridge Channel")
                    .build(),
            )
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
    pub async fn create_channel(
        &self,
        channel_value: impl Borrow<ChannelValue>,
    ) -> Result<Channel> {
        self.request(
            Method::POST,
            "/api/v0/channels",
            query_params![],
            request_body!(channel_value.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Updates a channel.
    ///
    /// See <https://mackerel.io/api-docs/entry/channels#update>.
    pub async fn update_channel(
        &self,
        channel_id: impl Into<ChannelId>,
        channel_value: impl Borrow<ChannelValue>,
    ) -> Result<Channel> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/channels/{}", channel_id),
            query_params![],
            request_body!(channel_value.borrow()),
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
        EmailChannel::builder()
            .name("Example Email Channel")
            .emails(["mackerel@example.com"])
            .user_ids(["user0"])
            .events([NotificationEvent::Alert])
            .build()
            .into()
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
            test_client!(server).create_channel(value_example()).await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server).create_channel(&value_example()).await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn update_channel() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/channels/channel0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_channel("channel0", value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_channel(ChannelId::from("channel0"), &value_example())
                .await,
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
