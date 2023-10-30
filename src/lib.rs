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
//! ```rust,no_run,ignore
//! let client = Client::new("<Mackerel-API-KEY>");
//!
//! println!("{:?}", client.get_organization().await);
//! println!("{:?}", client.list_users().await);
//! println!("{:?}", client.list_invitations().await);
//!
//! println!("{:?}", client.list_services().await);
//! println!("{:?}", client.list_service_metric_names("<Service-Name>".into()).await);
//!
//! println!("{:?}", client.list_monitors().await);
//! println!("{:?}", client.delete_monitor("<Monitor-ID>".into()).await);
//! ```
//!

pub mod client;
pub mod entity;
pub mod error;
pub(crate) mod name;

pub mod alert;
pub mod alert_group_setting;
pub mod channel;
pub mod dashboard;
pub mod downtime;
pub mod graph_annotation;
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
pub use crate::client::Client;

#[doc(hidden)]
#[macro_export]
macro_rules! response {
    { $( $field:ident: $type:ty ),+ $(,)? } => {{
        #[allow(non_snake_case)]
        #[derive(::serde_derive::Deserialize)]
        struct Response { $( $field: $type ),* }
        |response: Response| ($( response.$field ),*)
    }};
}
