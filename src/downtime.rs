use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::monitor::MonitorId;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};

/// A downtime
pub type Downtime = Entity<DowntimeValue>;

/// A downtime id
pub type DowntimeId = Id<DowntimeValue>;

/// A downtime value
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DowntimeValue {
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start: DateTime<Utc>,
    pub duration: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<DowntimeRecurrence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_exclude_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_exclude_scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_scopes: Vec<MonitorId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_exclude_scopes: Vec<MonitorId>,
}

/// A downtime recurrence
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DowntimeRecurrence {
    #[serde(rename = "type")]
    pub recurrence_type: DowntimeRecurrenceType,
    pub interval: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weekdays: Vec<DowntimeRecurrenceWeekday>,
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub until: Option<DateTime<Utc>>,
}

/// A downtime recurrence types
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "lowercase")]
pub enum DowntimeRecurrenceType {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// A downtime recurrence weekday
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
pub enum DowntimeRecurrenceWeekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[cfg(test)]
mod tests {
    use crate::downtime::*;
    use rstest::rstest;
    use serde_json::json;

    fn downtime_example1() -> Downtime {
        Downtime {
            id: "abcde1".into(),
            value: DowntimeValue {
                name: "Example downtime".to_string(),
                memo: "This is a downtime memo.".to_string(),
                start: DateTime::from_timestamp(1700000000, 0).unwrap(),
                duration: 60,
                recurrence: None,
                service_scopes: vec![],
                service_exclude_scopes: vec![],
                role_scopes: vec![],
                role_exclude_scopes: vec![],
                monitor_scopes: vec![],
                monitor_exclude_scopes: vec![],
            },
        }
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "name": "Example downtime",
            "memo": "This is a downtime memo.",
            "start": 1700000000,
            "duration": 60,
        })
    }

    fn downtime_example2() -> Downtime {
        Downtime {
            id: "abcde2".into(),
            value: DowntimeValue {
                name: "Example downtime".to_string(),
                memo: "".to_string(),
                start: DateTime::from_timestamp(1700000000, 0).unwrap(),
                duration: 60,
                recurrence: Some(DowntimeRecurrence {
                    recurrence_type: DowntimeRecurrenceType::Weekly,
                    interval: 30,
                    weekdays: vec![
                        DowntimeRecurrenceWeekday::Sunday,
                        DowntimeRecurrenceWeekday::Tuesday,
                        DowntimeRecurrenceWeekday::Wednesday,
                    ],
                    until: Some(DateTime::from_timestamp(1710000000, 0).unwrap()),
                }),
                service_scopes: vec!["service0".to_string()],
                service_exclude_scopes: vec!["service1".to_string()],
                role_scopes: vec!["service0:role0".to_string()],
                role_exclude_scopes: vec!["service1:role1".to_string()],
                monitor_scopes: vec!["monitor0".into()],
                monitor_exclude_scopes: vec!["monitor1".into()],
            },
        }
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "abcde2",
            "name": "Example downtime",
            "start": 1700000000,
            "duration": 60,
            "recurrence": {
                "type": "weekly",
                "interval": 30,
                "weekdays": ["Sunday", "Tuesday", "Wednesday"],
                "until": 1710000000,
            },
            "serviceScopes": ["service0"],
            "serviceExcludeScopes": ["service1"],
            "roleScopes": ["service0:role0"],
            "roleExcludeScopes": ["service1:role1"],
            "monitorScopes": ["monitor0"],
            "monitorExcludeScopes": ["monitor1"],
        })
    }

    #[test]
    fn serialize_downtime() {
        assert_eq!(
            json_example1(),
            serde_json::to_value(&downtime_example1()).unwrap()
        );
        assert_eq!(
            json_example2(),
            serde_json::to_value(&downtime_example2()).unwrap()
        );
    }

    #[test]
    fn deserialize_downtime() {
        assert_eq!(
            downtime_example1(),
            serde_json::from_value(json_example1()).unwrap()
        );
        assert_eq!(
            downtime_example2(),
            serde_json::from_value(json_example2()).unwrap()
        );
    }

    #[rstest]
    #[case(DowntimeRecurrenceType::Hourly, "hourly")]
    #[case(DowntimeRecurrenceType::Daily, "daily")]
    #[case(DowntimeRecurrenceType::Weekly, "weekly")]
    #[case(DowntimeRecurrenceType::Monthly, "monthly")]
    #[case(DowntimeRecurrenceType::Yearly, "yearly")]
    fn test_downtime_recurrence_type(
        #[case] downtime_recurrence_type: DowntimeRecurrenceType,
        #[case] downtime_recurrence_type_str: &str,
    ) {
        assert_eq!(
            downtime_recurrence_type.to_string(),
            downtime_recurrence_type_str
        );
        assert_eq!(
            downtime_recurrence_type,
            downtime_recurrence_type_str.parse().unwrap()
        );
        assert_eq!(
            downtime_recurrence_type,
            serde_json::from_value(downtime_recurrence_type_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(downtime_recurrence_type).unwrap(),
            downtime_recurrence_type_str
        );
    }

    #[rstest]
    #[case(DowntimeRecurrenceWeekday::Sunday, "Sunday")]
    #[case(DowntimeRecurrenceWeekday::Monday, "Monday")]
    #[case(DowntimeRecurrenceWeekday::Tuesday, "Tuesday")]
    #[case(DowntimeRecurrenceWeekday::Wednesday, "Wednesday")]
    #[case(DowntimeRecurrenceWeekday::Thursday, "Thursday")]
    #[case(DowntimeRecurrenceWeekday::Friday, "Friday")]
    #[case(DowntimeRecurrenceWeekday::Saturday, "Saturday")]
    fn test_downtime_recurrence_weekday(
        #[case] downtime_recurrence_weekday: DowntimeRecurrenceWeekday,
        #[case] downtime_recurrence_weekday_str: &str,
    ) {
        assert_eq!(
            downtime_recurrence_weekday.to_string(),
            downtime_recurrence_weekday_str
        );
        assert_eq!(
            downtime_recurrence_weekday,
            downtime_recurrence_weekday_str.parse().unwrap()
        );
        assert_eq!(
            downtime_recurrence_weekday,
            serde_json::from_value(downtime_recurrence_weekday_str.into()).unwrap()
        );
        assert_eq!(
            serde_json::to_value(downtime_recurrence_weekday).unwrap(),
            downtime_recurrence_weekday_str
        );
    }
}

#[derive(Deserialize)]
struct ListDowntimesResponse {
    downtimes: Vec<Downtime>,
}

impl client::Client {
    /// Fetches all the downtimes.
    ///
    /// See <https://mackerel.io/api-docs/entry/downtimes#list>.
    pub async fn list_downtimes(&self) -> Result<Vec<Downtime>> {
        self.request(
            Method::GET,
            "/api/v0/downtimes",
            vec![],
            client::empty_body(),
            |res: ListDowntimesResponse| res.downtimes,
        )
        .await
    }

    /// Creates a new downtime.
    ///
    /// See <https://mackerel.io/api-docs/entry/downtimes#create>.
    pub async fn create_downtime(&self, downtime_value: DowntimeValue) -> Result<Downtime> {
        self.request(
            Method::POST,
            "/api/v0/downtimes",
            vec![],
            Some(downtime_value),
            |downtime| downtime,
        )
        .await
    }

    /// Updates a downtime.
    ///
    /// See <https://mackerel.io/api-docs/entry/downtimes#update>.
    pub async fn update_downtime(
        &self,
        downtime_id: DowntimeId,
        downtime_value: DowntimeValue,
    ) -> Result<Downtime> {
        self.request(
            Method::PUT,
            format!("/api/v0/downtimes/{}", downtime_id),
            vec![],
            Some(downtime_value),
            |downtime| downtime,
        )
        .await
    }

    /// Deletes a downtime.
    ///
    /// See <https://mackerel.io/api-docs/entry/downtimes#delete>.
    pub async fn delete_downtime(&self, downtime_id: DowntimeId) -> Result<Downtime> {
        self.request(
            Method::DELETE,
            format!("/api/v0/downtimes/{}", downtime_id),
            vec![],
            client::empty_body(),
            |downtime| downtime,
        )
        .await
    }
}
