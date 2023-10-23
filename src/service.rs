use crate::client;
use crate::error::*;
use crate::role::RoleName;
use fixedstr::str64;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};

/// A service
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: ServiceName,
    pub memo: String,
    pub roles: Vec<RoleName>,
}

use std::marker::PhantomData;
#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct ServiceName {
    service_name: str64,
    phantom: PhantomData<Service>,
}

impl ServiceName {
    pub fn new(service_name: str64) -> Self {
        Self {
            service_name,
            phantom: PhantomData,
        }
    }
}

impl From<&str> for ServiceName {
    fn from(service_name: &str) -> Self {
        Self::new(service_name.into())
    }
}

impl From<String> for ServiceName {
    fn from(service_name: String) -> Self {
        Self::new(service_name.into())
    }
}

impl Into<String> for ServiceName {
    fn into(self: Self) -> String {
        self.service_name.to_string()
    }
}

impl std::ops::Deref for ServiceName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.service_name
    }
}

use std::fmt;
impl fmt::Display for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.service_name.fmt(f)
    }
}

impl fmt::Debug for ServiceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"")?;
        self.service_name.fmt(f)?;
        f.write_str("\"")
    }
}

use serde::ser::{Serialize, Serializer};
impl Serialize for ServiceName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.service_name.serialize(serializer)
    }
}

use serde::de::{Deserialize, Deserializer};
impl<'de> Deserialize<'de> for ServiceName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::new(str64::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::*;
    use serde_json::json;

    fn service_example() -> Service {
        Service {
            name: "FooService".into(),
            memo: "service memo".to_string(),
            roles: vec!["role0".into(), "role1".into(), "role2".into()],
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "FooService",
            "memo": "service memo",
            "roles": [
                "role0",
                "role1",
                "role2"
            ]
        })
    }

    #[test]
    fn serialize_service() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&service_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_service() {
        assert_eq!(
            service_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }
}

#[derive(Deserialize)]
struct ListServiceResponse {
    services: Vec<Service>,
}

#[derive(Deserialize)]
struct ListMetricNamesResponse {
    names: Vec<String>,
}

impl client::Client {
    /// Fetches all the services.
    ///
    /// See https://mackerel.io/api-docs/entry/services#list.
    pub async fn list_services(&self) -> Result<Vec<Service>> {
        self.request(
            Method::GET,
            "/api/v0/services",
            vec![],
            client::empty_body(),
            |res: ListServiceResponse| res.services,
        )
        .await
    }

    /// Creates a new service.
    ///
    /// See https://mackerel.io/api-docs/entry/services#create.
    pub async fn create_service(&self, service: Service) -> Result<Service> {
        self.request(
            Method::POST,
            "/api/v0/services",
            vec![],
            Some(service),
            |service| service,
        )
        .await
    }

    /// Deletes a service.
    ///
    /// See https://mackerel.io/api-docs/entry/services#delete.
    pub async fn delete_service(&self, service_name: ServiceName) -> Result<Service> {
        self.request(
            Method::DELETE,
            format!("/api/v0/services/{}", service_name),
            vec![],
            client::empty_body(),
            |service| service,
        )
        .await
    }

    /// Fetches service metric names.
    ///
    /// See https://mackerel.io/api-docs/entry/services#metric-names.
    pub async fn list_service_metric_names(
        &self,
        service_name: ServiceName,
    ) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metric-names", service_name),
            vec![],
            client::empty_body(),
            |res: ListMetricNamesResponse| res.names,
        )
        .await
    }
}
