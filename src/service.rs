use client;
use errors::*;
use reqwest::Method;

/// A service
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub memo: String,
    pub roles: Vec<String>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use service::*;

    fn service_example() -> Service {
        Service {
            name: "FooService".to_string(),
            memo: "service memo".to_string(),
            roles: vec![
                "role0".to_string(),
                "role1".to_string(),
                "role2".to_string(),
            ],
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
    pub fn list_services(&self) -> Result<Vec<Service>> {
        self.request(
            Method::GET,
            "/api/v0/services",
            vec![],
            client::empty_body(),
            |res: ListServiceResponse| res.services,
        )
    }

    /// Creates a new service.
    ///
    /// See https://mackerel.io/api-docs/entry/services#create.
    pub fn create_service(&self, service: Service) -> Result<Service> {
        self.request(
            Method::POST,
            "/api/v0/services",
            vec![],
            Some(service),
            |service| service,
        )
    }

    /// Deletes a service.
    ///
    /// See https://mackerel.io/api-docs/entry/services#delete.
    pub fn delete_service(&self, service_name: String) -> Result<Service> {
        self.request(
            Method::DELETE,
            format!("/api/v0/services/{}", service_name),
            vec![],
            client::empty_body(),
            |service| service,
        )
    }

    /// Fetches the names of the service metrics.
    ///
    /// See https://mackerel.io/api-docs/entry/services#metric-names.
    pub fn list_service_metric_names(&self, service_name: String) -> Result<Vec<String>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metric-names", service_name),
            vec![],
            client::empty_body(),
            |res: ListMetricNamesResponse| res.names,
        )
    }
}
