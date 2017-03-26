# mackerel-client-rs
[![Travis Build Status](https://travis-ci.org/itchyny/mackerel-client-rs.svg?branch=master)](https://travis-ci.org/itchyny/mackerel-client-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/mackerel_client.svg)](https://crates.io/crates/mackerel_client)

### An API client library for Mackerel written in Rust

## Example
```rust
extern crate mackerel_client;
use mackerel_client::*;

fn main() {
    let client = Client::new("<Mackerel-API-KEY>");

    println!("{:?}", client.get_organization());
    println!("{:?}", client.list_users());

    println!("{:?}", client.list_services());
    println!("{:?}", client.list_service_metric_names("<Service-Name>"));

    println!("{:?}", client.list_monitors());
    println!("{:?}", client.delete_monitor("<Monitor-ID>"));
}
```

## Author
itchyny <itchyny@hatena.ne.jp>

## License
This software is released under the MIT License, see LICENSE.
