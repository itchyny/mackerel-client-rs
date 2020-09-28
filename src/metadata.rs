use crate::{client, errors::*};
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
    pub fn get_metadata(&self, host_id: &str, namespace: &str) -> Result<serde_json::Value> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata/{}", host_id, namespace),
            vec![],
            client::empty_body(),
            |res| res,
        )
    }

    /// Creates/Updatates a host metadata.
    ///
    /// See https://mackerel.io/api-docs/entry/metadata#put.
    pub fn put_metadata(
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
    }

    /// Deletes a host metadata.
    ///
    /// See https://mackerel.io/api-docs/entry/metadata#delete.
    pub fn delete_metadata(&self, host_id: &str, namespace: &str) -> Result<()> {
        self.request(
            Method::DELETE,
            format!("/api/v0/hosts/{}/metadata/{}", host_id, namespace),
            vec![],
            client::empty_body(),
            |_: serde_json::Value| (),
        )
    }

    /// Lists host metadata.
    ///
    /// See https://mackerel.io/api-docs/entry/metadata#list.
    pub fn list_metadata(&self, host_id: &str) -> Result<Vec<Metadata>> {
        self.request(
            Method::GET,
            format!("/api/v0/hosts/{}/metadata", host_id),
            vec![],
            client::empty_body(),
            |res: ListMetadataResponse| res.metadata,
        )
    }
}
