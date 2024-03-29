//! An API client library for Mackerel
//!
//! [Mackerel](https://mackerel.io) is a performance monitoring and management tool of servers.
//! This monitoring SaaS provides you the intuitive user interfaces and useful APIs for automated infrastructure foundation.
//!
//! API documents: [Mackerel API Documents (v0)](https://mackerel.io/api-docs/)
//!
//! The official Go client library: [mackerel-client-go](https://github.com/mackerelio/mackerel-client-go)
//!
//! # Example
//!
//! ```rust,no_run
//! use mackerel_client::*;
//!
//! # #[async_std::main]
//! # async fn main() {
//! let client = Client::new("<Mackerel-API-KEY>");
//!
//! println!("{:?}", client.get_organization().await);
//! println!("{:?}", client.list_users().await);
//! println!("{:?}", client.list_invitations().await);
//!
//! println!("{:?}", client.list_services().await);
//! println!("{:?}", client.list_service_metric_names("<Service-Name>").await);
//!
//! println!("{:?}", client.list_monitors().await);
//! println!("{:?}", client.delete_monitor("<Monitor-ID>").await);
//! # }
//! ```
//!

pub mod client;
pub mod entity;
pub mod error;
pub(crate) mod name;
#[cfg(test)]
pub(crate) mod tests;

pub mod alert;
pub mod alert_group_setting;
pub mod aws_integration;
pub mod channel;
pub mod check_report;
pub mod dashboard;
pub mod downtime;
pub mod graph_annotation;
pub mod graph_definition;
pub mod host;
pub mod invitation;
pub mod metadata;
pub mod metric;
pub mod monitor;
pub mod notification_group;
pub mod organization;
pub mod role;
pub mod service;
pub mod user;

#[doc(inline)]
pub use crate::client::Client;
#[doc(inline)]
pub use crate::error::Error;
