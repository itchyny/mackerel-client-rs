use reqwest::Method::*;
use client;
use errors::*;

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
            roles: vec!["role0".to_string(), "role1".to_string(), "role2".to_string()],
        }
    }

    fn json_example() -> serde_json::Value {
        serde_json::from_str(r##"
            {
                "name": "FooService",
                "memo": "service memo",
                "roles": [
                    "role0",
                    "role1",
                    "role2"
                ]
            }
        "##)
            .unwrap()
    }

    #[test]
    fn serialize_service() {
        assert_eq!(json_example(), serde_json::to_value(&service_example()).unwrap());
    }

    #[test]
    fn deserialize_service() {
        assert_eq!(service_example(), serde_json::from_value(json_example()).unwrap());
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
        self.request(Get,
                     "/api/v0/services",
                     vec![],
                     client::empty_body(),
                     |res: ListServiceResponse| res.services)
    }

    /// Fetches the names of the service metrics.
    ///
    /// See https://mackerel.io/api-docs/entry/services#metric-names.
    pub fn list_service_metric_names(&self, service_name: &str) -> Result<Vec<String>> {
        self.request(Get,
                     format!("/api/v0/services/{}/metric-names", service_name),
                     vec![],
                     client::empty_body(),
                     |res: ListMetricNamesResponse| res.names)
    }
}
