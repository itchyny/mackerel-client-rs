use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json;

use crate::client::Client;
use crate::error::Result;
use crate::host::HostId;
use crate::macros::*;
use crate::role::RoleName;
use crate::service::ServiceName;

/// A metadata
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub namespace: String,
}

impl Client {
    /// Retrieves a host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostget>.
    pub async fn get_host_metadata(
        &self,
        host_id: impl Into<HostId>,
        namespace: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/hosts/{}/metadata/{}",
                host_id.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Creates/Updates a host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostput>.
    pub async fn put_host_metadata(
        &self,
        host_id: impl Into<HostId>,
        namespace: impl AsRef<str>,
        metadata: &serde_json::Value,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!(
                "/api/v0/hosts/{}/metadata/{}",
                host_id.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body!(metadata),
            response_body!(),
        )
        .await
    }

    /// Deletes a host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostdelete>.
    pub async fn delete_host_metadata(
        &self,
        host_id: impl Into<HostId>,
        namespace: impl AsRef<str>,
    ) -> Result<()> {
        self.request(
            Method::DELETE,
            format!(
                "/api/v0/hosts/{}/metadata/{}",
                host_id.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body![],
            response_body!(),
        )
        .await
    }

    /// Lists host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostlist>.
    pub async fn list_host_metadata(&self, host_id: impl Into<HostId>) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata", host_id.into()),
            query_params![],
            request_body![],
            response_body! { metadata: Vec<Metadata> },
        )
        .await
    }

    /// Retrieves a service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#serviceget>.
    pub async fn get_service_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        namespace: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/services/{}/metadata/{}",
                service_name.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Creates/Updates a service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#serviceput>.
    pub async fn put_service_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        namespace: impl AsRef<str>,
        metadata: &serde_json::Value,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!(
                "/api/v0/services/{}/metadata/{}",
                service_name.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body!(metadata),
            response_body!(),
        )
        .await
    }

    /// Deletes a service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#servicedelete>.
    pub async fn delete_service_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        namespace: impl AsRef<str>,
    ) -> Result<()> {
        self.request(
            Method::DELETE,
            format!(
                "/api/v0/services/{}/metadata/{}",
                service_name.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body![],
            response_body!(),
        )
        .await
    }

    /// Lists service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#servicelist>.
    pub async fn list_service_metadata(
        &self,
        service_name: impl Into<ServiceName>,
    ) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metadata", service_name.into()),
            query_params![],
            request_body![],
            response_body! { metadata: Vec<Metadata> },
        )
        .await
    }

    /// Retrieves a role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#roleget>.
    pub async fn get_role_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        role_name: impl Into<RoleName>,
        namespace: impl AsRef<str>,
    ) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/services/{}/roles/{}/metadata/{}",
                service_name.into(),
                role_name.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Creates/Updates a role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#roleput>.
    pub async fn put_role_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        role_name: impl Into<RoleName>,
        namespace: impl AsRef<str>,
        metadata: &serde_json::Value,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!(
                "/api/v0/services/{}/roles/{}/metadata/{}",
                service_name.into(),
                role_name.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body!(metadata),
            response_body!(),
        )
        .await
    }

    /// Deletes a role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#roledelete>.
    pub async fn delete_role_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        role_name: impl Into<RoleName>,
        namespace: impl AsRef<str>,
    ) -> Result<()> {
        self.request(
            Method::DELETE,
            format!(
                "/api/v0/services/{}/roles/{}/metadata/{}",
                service_name.into(),
                role_name.into(),
                namespace.as_ref()
            ),
            query_params![],
            request_body![],
            response_body!(),
        )
        .await
    }

    /// Lists role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#rolelist>.
    pub async fn list_role_metadata(
        &self,
        service_name: impl Into<ServiceName>,
        role_name: impl Into<RoleName>,
    ) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/services/{}/roles/{}/metadata",
                service_name.into(),
                role_name.into(),
            ),
            query_params![],
            request_body![],
            response_body! { metadata: Vec<Metadata> },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::metadata::*;
    use crate::tests::*;

    fn metadata_value_example() -> serde_json::Value {
        json!({ "test": "This is a metadata example." })
    }

    fn metadata_example() -> Metadata {
        Metadata {
            namespace: "namespace0".to_owned(),
        }
    }

    fn metadata_json_example() -> serde_json::Value {
        json!({ "namespace": "namespace0" })
    }

    #[async_std::test]
    async fn get_host_metadata() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts/host0/metadata/namespace0",
            response = metadata_value_example(),
        };
        assert_eq!(
            test_client!(server)
                .get_host_metadata("host0", "namespace0")
                .await,
            Ok(metadata_value_example())
        );
    }

    #[async_std::test]
    async fn put_host_metadata() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/hosts/host0/metadata/namespace0",
            request = metadata_value_example(),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .put_host_metadata("host0", "namespace0", &metadata_value_example())
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn delete_host_metadata() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/hosts/host0/metadata/namespace0",
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .delete_host_metadata("host0", "namespace0")
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn list_host_metadata() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/hosts/host0/metadata",
            response = json!({
                "metadata": [metadata_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_host_metadata("host0").await,
            Ok(vec![metadata_example()])
        );
    }

    #[async_std::test]
    async fn get_service_metadata() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/metadata/namespace0",
            response = metadata_value_example(),
        };
        assert_eq!(
            test_client!(server)
                .get_service_metadata("service0", "namespace0")
                .await,
            Ok(metadata_value_example())
        );
    }

    #[async_std::test]
    async fn put_service_metadata() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/services/service0/metadata/namespace0",
            request = metadata_value_example(),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .put_service_metadata("service0", "namespace0", &metadata_value_example())
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn delete_service_metadata() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/services/service0/metadata/namespace0",
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .delete_service_metadata("service0", "namespace0")
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn list_service_metadata() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/metadata",
            response = json!({
                "metadata": [metadata_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_service_metadata("service0").await,
            Ok(vec![metadata_example()])
        );
    }

    #[async_std::test]
    async fn get_role_metadata() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/roles/role0/metadata/namespace0",
            response = metadata_value_example(),
        };
        assert_eq!(
            test_client!(server)
                .get_role_metadata("service0", "role0", "namespace0")
                .await,
            Ok(metadata_value_example())
        );
    }

    #[async_std::test]
    async fn put_role_metadata() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/services/service0/roles/role0/metadata/namespace0",
            request = metadata_value_example(),
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .put_role_metadata("service0", "role0", "namespace0", &metadata_value_example())
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn delete_role_metadata() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/services/service0/roles/role0/metadata/namespace0",
            response = json!({ "success": true }),
        };
        assert_eq!(
            test_client!(server)
                .delete_role_metadata("service0", "role0", "namespace0")
                .await,
            Ok(())
        );
    }

    #[async_std::test]
    async fn list_role_metadata() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/services/service0/roles/role0/metadata",
            response = json!({
                "metadata": [metadata_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_role_metadata("service0", "role0")
                .await,
            Ok(vec![metadata_example()])
        );
    }
}
