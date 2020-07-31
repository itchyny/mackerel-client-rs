# mackerel-client-rs [![CI Status][ci-badge]][ci-url] [![MIT License][license-badge]][license-url] [![crates.io][cratesio-badge]][cratesio-url]
### An API client library for Mackerel written in Rust
[Mackerel](https://mackerel.io) is a performance monitoring and management tool of servers.
This monitoring SaaS provides you the intuitive user interfaces and useful APIs for automated infrastructure foundation.

API documents: [Mackerel API Documents (v0)](https://mackerel.io/api-docs/)

The official Go client library: [mackerel-client-go](https://github.com/mackerelio/mackerel-client-go)

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

Refer to the [documentation](https://docs.rs/mackerel_client/).

## Author
itchyny <itchyny@hatena.ne.jp>

## License
This software is released under the MIT License, see LICENSE.

[ci-url]: https://github.com/itchyny/mackerel-client-rs/actions
[ci-badge]: https://github.com/itchyny/mackerel-client-rs/workflows/CI/badge.svg
[license-url]: https://github.com/itchyny/mackerel-client-rs/blob/master/LICENSE
[license-badge]: http://img.shields.io/badge/license-MIT-blue.svg
[cratesio-url]: https://crates.io/crates/mackerel_client
[cratesio-badge]: https://img.shields.io/crates/v/mackerel_client.svg
