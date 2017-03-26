//! An API client library for Mackerel
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

pub mod organization;
pub mod role;
pub mod service;
pub mod user;
