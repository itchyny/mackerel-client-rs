# mackerel-client-rs
[![CI Status](https://github.com/itchyny/mackerel-client-rs/workflows/CI/badge.svg)](https://github.com/itchyny/mackerel-client-rs/actions)
[![crates.io](https://img.shields.io/crates/v/mackerel-client.svg)](https://crates.io/crates/mackerel-client)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/itchyny/mackerel-client-rs/blob/master/LICENSE)

### An API client library for Mackerel written in Rust
[Mackerel](https://mackerel.io) is a performance monitoring and management tool of servers.
This monitoring SaaS provides you the intuitive user interfaces and useful APIs for automated infrastructure foundation.

API documents: [Mackerel API Documents (v0)](https://mackerel.io/api-docs/)

The official Go client library: [mackerel-client-go](https://github.com/mackerelio/mackerel-client-go)

## Example
```rust
use mackerel_client::*;

#[async_std::main]
async fn main() {
    let client = Client::new("<Mackerel-API-KEY>");

    println!("{:?}", client.get_organization().await);
    println!("{:?}", client.list_users().await);

    println!("{:?}", client.list_services().await);
    println!("{:?}", client.list_service_metric_names("<Service-Name>").await);

    println!("{:?}", client.list_monitors().await);
    println!("{:?}", client.delete_monitor("<Monitor-ID>").await);
}
```

Refer to the [documentation](https://docs.rs/mackerel_client/).

## Author
itchyny <itchyny@hatena.ne.jp>

## License
This software is released under the MIT License, see LICENSE.
