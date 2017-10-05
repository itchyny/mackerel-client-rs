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
//! println!("{:?}", client.get_organization());
//! println!("{:?}", client.list_users());
//!
//! println!("{:?}", client.list_services());
//! println!("{:?}", client.list_service_metric_names("<Service-Name>"));
//!
//! println!("{:?}", client.list_monitors());
//! println!("{:?}", client.delete_monitor("<Monitor-ID>"));
//! ```
//!
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate reqwest;
extern crate url;

#[macro_use]
extern crate error_chain;
pub mod errors {
    use reqwest;
    error_chain!{
        errors {
            ApiError(status: reqwest::StatusCode, message: String)
        }
    }
}

pub mod client;

pub mod alert;
pub mod dashboard;
pub mod graph_annotation;
pub mod invitation;
pub mod metadata;
pub mod monitor;
pub mod organization;
pub mod role;
pub mod service;
pub mod user;
pub use client::Client;
