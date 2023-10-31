use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::client::Client;
use crate::error::Result;
use crate::macros::*;
use crate::name::Name;
use crate::role::RoleName;

/// A service
#[derive(PartialEq, Eq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct Service {
    pub name: ServiceName,
    #[builder(default)]
    pub memo: String,
    #[builder(default)]
    pub roles: Vec<RoleName>,
}

/// A service name
/// ```rust
/// use mackerel_client::service::ServiceName;
///
/// let service_name = ServiceName::from("ExampleService");
/// ```
pub type ServiceName = Name<Service>;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn service_example1() -> Service {
        Service::builder()
            .name("ExampleService1")
            .memo("service memo")
            .roles(["role0".into(), "role1".into(), "role2".into()])
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "name": "ExampleService1",
            "memo": "service memo",
            "roles": [
                "role0",
                "role1",
                "role2"
            ]
        })
    }

    fn service_example2() -> Service {
        Service::builder().name("ExampleService2").build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "name": "ExampleService2",
            "memo": "",
            "roles": [],
        })
    }

    #[rstest]
    #[case(service_example1(), json_example1())]
    #[case(service_example2(), json_example2())]
    fn test_service_json(#[case] service: Service, #[case] json: serde_json::Value) {
        assert_eq!(serde_json::to_value(&service).unwrap(), json);
        assert_eq!(service, serde_json::from_value(json).unwrap());
    }
}

impl Client {
    /// Fetches all the services.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#list>.
    pub async fn list_services(&self) -> Result<Vec<Service>> {
        self.request(
            Method::GET,
            "/api/v0/services",
            query_params![],
            request_body![],
            response_body! { services: Vec<Service> },
        )
        .await
    }

    /// Creates a new service.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#create>.
    pub async fn create_service(&self, service: Service) -> Result<Service> {
        self.request(
            Method::POST,
            "/api/v0/services",
            query_params![],
            request_body!(service),
            response_body!(..),
        )
        .await
    }

    /// Deletes a service.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#delete>.
    pub async fn delete_service(&self, service_name: ServiceName) -> Result<Service> {
        self.request(
            Method::DELETE,
            format!("/api/v0/services/{}", service_name),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Fetches service metric names.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#metric-names>.
    pub async fn list_service_metric_names(
        &self,
        service_name: ServiceName,
    ) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metric-names", service_name),
            query_params![],
            request_body![],
            response_body! { names: Vec<String> },
        )
        .await
    }
}
