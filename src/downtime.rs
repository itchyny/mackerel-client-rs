use crate::client;
use crate::entity::{Entity, Id};
use crate::error::*;
use crate::monitor::MonitorId;
use crate::role::RoleFullname;
use crate::service::ServiceName;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

/// A downtime
pub type Downtime = Entity<DowntimeValue>;

/// A downtime id
pub type DowntimeId = Id<DowntimeValue>;

/// A downtime value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct DowntimeValue {
    pub name: String,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub memo: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub start: DateTime<Utc>,
    #[builder(setter(!into))]
    pub duration: u64,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<DowntimeRecurrence>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_scopes: Vec<ServiceName>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_exclude_scopes: Vec<ServiceName>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_scopes: Vec<RoleFullname>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_exclude_scopes: Vec<RoleFullname>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_scopes: Vec<MonitorId>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_exclude_scopes: Vec<MonitorId>,
}

/// A downtime recurrence
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct DowntimeRecurrence {
    #[serde(rename = "type")]
    pub recurrence_type: DowntimeRecurrenceType,
    #[builder(setter(!into))]
    pub interval: u64,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub weekdays: Vec<DowntimeRecurrenceWeekday>,
    #[builder(default, setter(strip_option))]
    #[serde(default, with = "chrono::serde::ts_seconds_option")]
    pub until: Option<DateTime<Utc>>,
}

/// A downtime recurrence type
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
        Downtime::builder()
            .id("abcde1")
            .value(
                DowntimeValue::builder()
                    .name("Example downtime")
                    .memo("This is a downtime memo.")
                    .start(DateTime::from_timestamp(1700000000, 0).unwrap())
                    .duration(60)
                    .build(),
            )
            .build()
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
        Downtime::builder()
            .id("abcde2")
            .value(
                DowntimeValue::builder()
                    .name("Example downtime")
                    .start(DateTime::from_timestamp(1700000000, 0).unwrap())
                    .duration(60)
                    .recurrence(
                        DowntimeRecurrence::builder()
                            .recurrence_type(DowntimeRecurrenceType::Weekly)
                            .interval(30)
                            .weekdays([
                                DowntimeRecurrenceWeekday::Sunday,
                                DowntimeRecurrenceWeekday::Tuesday,
                                DowntimeRecurrenceWeekday::Wednesday,
                            ])
                            .until(DateTime::from_timestamp(1710000000, 0).unwrap())
                            .build(),
                    )
                    .service_scopes(["service0".into()])
                    .service_exclude_scopes(["service1".into()])
                    .role_scopes(["service0:role0".into()])
                    .role_exclude_scopes(["service1:role1".into()])
                    .monitor_scopes(["monitor0".into()])
                    .monitor_exclude_scopes(["monitor1".into()])
                    .build(),
            )
            .build()
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
