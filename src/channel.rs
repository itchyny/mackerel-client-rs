use crate::client;
use crate::errors::*;
use reqwest::Method;

/// A channel
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub channel_type: String,
}

#[cfg(test)]
mod tests {
    use crate::channel::*;
    use serde_json;

    fn channel_example1() -> Channel {
        Channel {
            id: "abcde1".to_string(),
            name: "Example Channel 1".to_string(),
            channel_type: "slack".to_string(),
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "name": "Example Channel 1",
            "type": "slack"
        })
    }

    fn channel_example2() -> Channel {
        Channel {
            id: "abcde2".to_string(),
            name: "Example Channel 2".to_string(),
            channel_type: "email".to_string(),
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "name": "Example Channel 2",
            "type": "email"
        })
    }

    #[test]
    fn serialize_channel() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&channel_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&channel_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_channel() {
        assert_eq!(
            channel_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            channel_example2(),
            serde_json::from_value(json_example2()).unwrap()
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
    /// See https://mackerel.io/api-docs/entry/channels#get.
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
}
