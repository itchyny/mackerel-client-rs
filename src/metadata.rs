use crate::client;
use crate::error::*;
use crate::host::HostId;
use crate::role::RoleName;
use crate::service::ServiceName;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_json;

/// A metadata
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub namespace: String,
}

#[derive(Deserialize)]
struct ListMetadataResponse {
    metadata: Vec<Metadata>,
}

impl client::Client {
    /// Retrieves a host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostget>.
    pub async fn get_host_metadata(
        &self,
        host_id: HostId,
        namespace: String,
    ) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata/{}", host_id, namespace),
            vec![],
            client::empty_body(),
            |res| res,
        )
        .await
    }

    /// Creates/Updates a host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostput>.
    pub async fn put_host_metadata(
        &self,
        host_id: HostId,
        namespace: String,
        metadata: serde_json::Value,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/hosts/{}/metadata/{}", host_id, namespace),
            vec![],
            Some(metadata),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Deletes a host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostdelete>.
    pub async fn delete_host_metadata(&self, host_id: HostId, namespace: String) -> Result<()> {
        self.request(
            Method::DELETE,
            format!("/api/v0/hosts/{}/metadata/{}", host_id, namespace),
            vec![],
            client::empty_body(),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Lists host metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#hostlist>.
    pub async fn list_host_metadata(&self, host_id: HostId) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata", host_id),
            vec![],
            client::empty_body(),
            |res: ListMetadataResponse| res.metadata,
        )
        .await
    }

    /// Retrieves a service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#serviceget>.
    pub async fn get_service_metadata(
        &self,
        service_name: ServiceName,
        namespace: String,
    ) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metadata/{}", service_name, namespace),
            vec![],
            client::empty_body(),
            |res| res,
        )
        .await
    }

    /// Creates/Updates a service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#serviceput>.
    pub async fn put_service_metadata(
        &self,
        service_name: ServiceName,
        namespace: String,
        metadata: serde_json::Value,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!("/api/v0/services/{}/metadata/{}", service_name, namespace),
            vec![],
            Some(metadata),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Deletes a service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#servicedelete>.
    pub async fn delete_service_metadata(
        &self,
        service_name: ServiceName,
        namespace: String,
    ) -> Result<()> {
        self.request(
            Method::DELETE,
            format!("/api/v0/services/{}/metadata/{}", service_name, namespace),
            vec![],
            client::empty_body(),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Lists service metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#servicelist>.
    pub async fn list_service_metadata(&self, service_name: ServiceName) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!("/api/v0/services/{}/metadata", service_name),
            vec![],
            client::empty_body(),
            |res: ListMetadataResponse| res.metadata,
        )
        .await
    }

    /// Retrieves a role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#roleget>.
    pub async fn get_role_metadata(
        &self,
        service_name: ServiceName,
        role_name: RoleName,
        namespace: String,
    ) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/services/{}/roles/{}/metadata/{}",
                service_name, role_name, namespace
            ),
            vec![],
            client::empty_body(),
            |res| res,
        )
        .await
    }

    /// Creates/Updates a role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#roleput>.
    pub async fn put_role_metadata(
        &self,
        service_name: ServiceName,
        role_name: RoleName,
        namespace: String,
        metadata: serde_json::Value,
    ) -> Result<()> {
        self.request(
            Method::PUT,
            format!(
                "/api/v0/services/{}/roles/{}/metadata/{}",
                service_name, role_name, namespace
            ),
            vec![],
            Some(metadata),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Deletes a role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#roledelete>.
    pub async fn delete_role_metadata(
        &self,
        service_name: ServiceName,
        role_name: RoleName,
        namespace: String,
    ) -> Result<()> {
        self.request(
            Method::DELETE,
            format!(
                "/api/v0/services/{}/roles/{}/metadata/{}",
                service_name, role_name, namespace
            ),
            vec![],
            client::empty_body(),
            |_: serde_json::Value| (),
        )
        .await
    }

    /// Lists role metadata.
    ///
    /// See <https://mackerel.io/api-docs/entry/metadata#rolelist>.
    pub async fn list_role_metadata(
        &self,
        service_name: ServiceName,
        role_name: RoleName,
    ) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!(
                "/api/v0/services/{}/roles/{}/metadata",
                service_name, role_name,
            ),
            vec![],
            client::empty_body(),
            |res: ListMetadataResponse| res.metadata,
        )
        .await
    }
}
