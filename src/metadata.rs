use crate::client;
use crate::errors::*;
use reqwest::Method;
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
    /// See https://mackerel.io/api-docs/entry/metadata#get.
    pub async fn get_metadata(&self, host_id: &str, namespace: &str) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata/{}", host_id, namespace),
            vec![],
            client::empty_body(),
            |res| res,
        )
        .await
    }

    /// Creates/Updatates a host metadata.
    ///
    /// See https://mackerel.io/api-docs/entry/metadata#put.
    pub async fn put_metadata(
        &self,
        host_id: &str,
        namespace: &str,
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
    /// See https://mackerel.io/api-docs/entry/metadata#delete.
    pub async fn delete_metadata(&self, host_id: &str, namespace: &str) -> Result<()> {
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
    /// See https://mackerel.io/api-docs/entry/metadata#list.
    pub async fn list_metadata(&self, host_id: &str) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata", host_id),
            vec![],
            client::empty_body(),
            |res: ListMetadataResponse| res.metadata,
        )
        .await
    }
}
