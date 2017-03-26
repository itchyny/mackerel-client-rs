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
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate error_chain;
pub mod errors {
    use hyper;
    error_chain!{
        errors {
            ApiError(status: hyper::status::StatusCode, message: String)
        }
    }
}

pub mod client;

pub mod alert;
pub mod monitor;
pub mod organization;
pub mod role;
pub mod service;
pub mod user;
pub use client::Client;
