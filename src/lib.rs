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
//!
//! println!("{:?}", client.list_services().await);
//! println!("{:?}", client.list_service_metric_names("<Service-Name>").await);
//!
//! println!("{:?}", client.list_monitors().await);
//! println!("{:?}", client.delete_monitor("<Monitor-ID>").await);
//! ```
//!

pub mod errors {
    use reqwest;
    use std::fmt;

    #[derive(Debug)]
    pub enum Error {
        ApiError(reqwest::StatusCode, String),
        MsgError(String),
    }

    impl From<String> for Error {
        fn from(s: String) -> Self {
            Error::MsgError(s)
        }
    }

    impl From<&str> for Error {
        fn from(s: &str) -> Self {
            Error::MsgError(s.to_string())
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::ApiError(status, message) => {
                    write!(f, "status:{}, message:{}", status, message)
                }
                Error::MsgError(message) => write!(f, "message:{}", message),
            }
        }
    }

    impl std::error::Error for Error {}

    pub type Result<T> = std::result::Result<T, Error>;
}

#[macro_use]
mod macros;

pub mod client;

pub mod alert;
pub mod authority;
pub mod channel;
pub mod dashboard;
pub mod graph_annotation;
pub mod host;
pub mod invitation;
pub mod metadata;
pub mod monitor;
pub mod organization;
pub mod role;
pub mod service;
pub mod user;
pub use crate::client::Client;
