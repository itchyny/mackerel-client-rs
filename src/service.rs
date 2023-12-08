use http::Method;
use serde_derive::{Deserialize, Serialize};
use std::borrow::Borrow;
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::name::Name;
use crate::role::RoleName;

/// A service value
#[derive(PartialEq, Eq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct Service {
    pub name: ServiceName,
    #[builder(default)]
    pub memo: String,
    #[builder(
        default,
        setter(transform = |role_names: impl IntoIterator<Item = impl Into<RoleName>>| role_names
            .into_iter().map(Into::into).collect::<Vec<_>>()),
    )]
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
            .name("service1")
            .memo("service memo")
            .roles(["role0", "role1", "role2"])
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "name": "service1",
            "memo": "service memo",
            "roles": ["role0", "role1", "role2"],
        })
    }

    fn service_example2() -> Service {
        Service::builder().name("service2").build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "name": "service2",
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
    pub async fn create_service(&self, service: impl Borrow<Service>) -> Result<Service> {
        self.request(
            Method::POST,
            "/api/v0/services",
            query_params![],
            request_body!(service.borrow()),
            response_body!(..),
        )
        .await
    }

    /// Deletes a service.
    ///
    /// See <https://mackerel.io/api-docs/entry/services#delete>.
    pub async fn delete_service(&self, service_name: impl Into<ServiceName>) -> Result<Service> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/services/{}", service_name),
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
        service_name: impl Into<ServiceName>,
    ) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format_url!("/api/v0/services/{}/metric-names", service_name),
            query_params![],
            request_body![],
            response_body! { names: Vec<String> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::service::*;
    use crate::tests::*;

    fn value_example() -> Service {
        Service::builder()
            .name("service0")
            .memo("This is a service memo.")
            .build()
    }

    fn json_example() -> serde_json::Value {
        json!({
            "name": "service0",
            "memo": "This is a service memo.",
            "roles": [],
        })
    }

    #[async_std::test]
    async fn list_services() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services",
            response = json!({
                "services": [json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_services().await,
            Ok(vec![value_example()]),
        );
    }

    #[async_std::test]
    async fn create_service() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/services",
            request = json_example(),
            response = json_example(),
        };
        assert_eq!(
            test_client!(server).create_service(value_example()).await,
            Ok(value_example()),
        );
        assert_eq!(
            test_client!(server).create_service(&value_example()).await,
            Ok(value_example()),
        );
    }

    #[async_std::test]
    async fn delete_service() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/services/service0",
            response = json_example(),
        };
        assert_eq!(
            test_client!(server).delete_service("service0").await,
            Ok(value_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_service(ServiceName::from("service0"))
                .await,
            Ok(value_example()),
        );
    }

    #[async_std::test]
    async fn list_service_metric_names() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/metric-names",
            response = json!({
                "names": [
                    "custom.service.metric0",
                    "custom.service.metric1",
                    "custom.service.metric2",
                ],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_service_metric_names("service0")
                .await,
            Ok(vec![
                "custom.service.metric0".to_owned(),
                "custom.service.metric1".to_owned(),
                "custom.service.metric2".to_owned(),
            ]),
        );
        assert_eq!(
            test_client!(server)
                .list_service_metric_names(ServiceName::from("service0"))
                .await,
            Ok(vec![
                "custom.service.metric0".to_owned(),
                "custom.service.metric1".to_owned(),
                "custom.service.metric2".to_owned(),
            ]),
        );
    }
}
