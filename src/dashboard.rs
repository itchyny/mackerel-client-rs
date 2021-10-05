use crate::client;
use crate::errors::*;
use reqwest::Method;

/// A dashboard
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dashboard {
    pub id: Option<String>,
    pub title: String,
    pub body_markdown: String,
    pub url_path: String,
}

#[cfg(test)]
mod tests {
    use crate::dashboard::*;
    use serde_json;

    fn dashboard_example() -> Dashboard {
        Dashboard {
            id: Some("abcde1".to_string()),
            title: "This is a dashboard".to_string(),
            body_markdown: "# Example\n[example](https://example.com)".to_string(),
            url_path: "example".to_string(),
        }
    }

    fn json_example() -> serde_json::Value {
        json!({
            "id": "abcde1",
            "title": "This is a dashboard",
            "bodyMarkdown": "# Example\n[example](https://example.com)",
            "urlPath": "example"
        })
    }

    #[test]
    fn serialize_dashboard() {
        assert_eq!(
            json_example(),
            serde_json::to_value(&dashboard_example()).unwrap()
        );
    }

    #[test]
    fn deserialize_dashboard() {
        assert_eq!(
            dashboard_example(),
            serde_json::from_value(json_example()).unwrap()
        );
    }
}

#[derive(Deserialize)]
struct ListDashboardsResponse {
    dashboards: Vec<Dashboard>,
}

impl client::Client {
    /// Fetches all the dashboards.
    ///
    /// See https://mackerel.io/api-docs/entry/dashboards#list.
    pub fn list_dashboards(&self) -> Result<Vec<Dashboard>> {
        self.request(
            Method::GET,
            "/api/v0/dashboards",
            vec![],
            client::empty_body(),
            |res: ListDashboardsResponse| res.dashboards,
        )
    }

    /// Creates a new dashboard.
    ///
    /// See https://mackerel.io/api-docs/entry/dashboards#create.
    pub fn create_dashboard(&self, dashboard: Dashboard) -> Result<Dashboard> {
        self.request(
            Method::POST,
            "/api/v0/dashboards",
            vec![],
            Some(dashboard),
            |dashboard| dashboard,
        )
    }

    /// Gets a dashboard.
    ///
    /// See https://mackerel.io/api-docs/entry/dashboards#get.
    pub fn get_dashboard(&self, dashboard_id: String) -> Result<Dashboard> {
        self.request(
            Method::GET,
            format!("/api/v0/dashboards/{}", dashboard_id),
            vec![],
            client::empty_body(),
            |dashboard| dashboard,
        )
    }

    /// Updates a dashboard.
    ///
    /// See https://mackerel.io/api-docs/entry/dashboards#update.
    pub fn update_dashboard(&self, dashboard: Dashboard) -> Result<Dashboard> {
        let dashboard_id: String = dashboard
            .clone()
            .id
            .ok_or("specify the id to update a dashboard")?;
        self.request(
            Method::PUT,
            format!("/api/v0/dashboards/{}", dashboard_id),
            vec![],
            Some(dashboard),
            |dashboard| dashboard,
        )
    }

    /// Deletes a dashboard.
    ///
    /// See https://mackerel.io/api-docs/entry/dashboards#delete.
    pub fn delete_dashboard(&self, dashboard_id: String) -> Result<Dashboard> {
        self.request(
            Method::DELETE,
            format!("/api/v0/dashboards/{}", dashboard_id),
            vec![],
            client::empty_body(),
            |dashboard| dashboard,
        )
    }
}
