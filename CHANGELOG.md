# Changelog
## [v0.7.0](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.6..v0.7.0) (2026-11-18)
* implement logs APIs: `list_logs` and `list_saved_log_searches`
* implement traces APIs: `list_traces` and `get_trace`
* implement APM APIs: `list_http_server_stats` and `list_db_query_stats`
* implement check monitor APIs: `list_check_monitors` and `get_check_monitor`
* implement alert logs API, update channel API, and delete service graph definition API
* split the variants of monitors, channels, and dashboard widgets into non-exhaustive
  structs with builders, so that fields can be added without breaking changes
* support email broadcast and Jira Service Management channels, and remove LINE, Typetalk,
  and Yammer channels
* support `expected_status_code` and `dualstack` of external monitors,
  `evaluate_backward_minutes` of expression and query monitors, `created_at`, `updated_at`,
  and `legend_list` of dashboards, `secret_key` of AWS integrations, and
  `add_to_default_notification_group` of channels
* add MemoryDB to AWS integration services
* make `legend` of query monitors and `name` of connectivity monitors optional
* fix `suspended_at` of channels other than email, Slack, and webhook channels
* fix percent-encoding of the custom identifier of `get_host_by_custom_identifier`
* add `retry_after` to `Error::ApiError` from the `Retry-After` header, and change it to a
  struct variant
* add `Error::DecodeError` for unexpected response bodies, and make `Error` non-exhaustive
* make the fields of `ExternalHeader` public
* rename `check_report` module to `check`
* update `reqwest` to 0.13, and replace `async-std` with `tokio` in the tests and the examples

## [v0.6.6](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.5..v0.6.6) (2024-10-06)
* add Athena to AWS integration services

## [v0.6.5](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.4..v0.6.5) (2024-06-05)
* fix the path of delete graph definition API

## [v0.6.4](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.3..v0.6.4) (2024-05-30)
* implement delete graph definition API

## [v0.6.3](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.2..v0.6.3) (2024-04-07)
* implement query graphs and query metrics of dashboard widgets

## [v0.6.2](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.1..v0.6.2) (2024-04-05)
* implement query monitor

## [v0.6.1](https://github.com/itchyny/mackerel-client-rs/compare/v0.6.0..v0.6.1) (2024-03-25)
* support `suspended_at` of channels
* support display name of organization

## [v0.6.0](https://github.com/itchyny/mackerel-client-rs/compare/v0.5.0..v0.6.0) (2023-12-15)
* implement AWS integration APIs, host monitored statuses API, latest host metric values
  API, getting a monitor API, creating graph definition API, and check report API
* implement listing alerts with cursors
* implement `ListHostsParams` to list hosts by service, roles, name, and statuses
* implement channel values by enum types, and creating and deleting channel APIs
* implement `ServiceName`, `RoleName`, `RoleFullname`, and `MonitorScope` types, and use
  fixed size strings to allow copying ids and names
* support thresholds for interruption monitoring of service metric monitors, request body
  and redirects of external monitors, max check attempts of metric monitors, and alert
  status on gone of connectivity monitors
* support value ranges, reference lines, and format rules of dashboard widgets
* use `chrono::DateTime` for date time fields
* implement builders of types, and `Client::builder` to configure the API base, the user
  agent, and the timeout
* accept owned and borrowed values for the arguments of `Client` methods
* reuse the HTTP client for connection pooling, set the default headers and the timeout, and
  disable redirects
* refactor `Error` type using `thiserror` crate

## [v0.5.0](https://github.com/itchyny/mackerel-client-rs/compare/v0.4.0..v0.5.0) (2023-10-21)
* make the client asynchronous
* implement host APIs, bulk host APIs, custom identifier API, posting and fetching metric
  value APIs, all metadata APIs, notification group APIs, alert group setting APIs,
  downtime APIs, and invitation list API
* implement `Entity` and `Id` types to distinguish entities, value objects, and ids of each
  type
* support widgets of dashboards
* send `User-Agent` header with the version of the library
* remove dependency on error-chain crate
* update to 2021 edition

## [v0.4.0](https://github.com/itchyny/mackerel-client-rs/compare/v0.3.0..v0.4.0) (2020-07-25)
* implement anomaly detection monitor
* implement creating host API

## [v0.3.0](https://github.com/itchyny/mackerel-client-rs/compare/v0.2.0..v0.3.0) (2019-03-30)
* implement role APIs
* make warning and critical thresholds of monitors optional
* rename `create_services` to `create_service`

## [v0.2.0](https://github.com/itchyny/mackerel-client-rs/compare/v0.1.0..v0.2.0) (2018-08-19)
* implement channel API, and creating and deleting service APIs
* support authority of users

## [v0.1.0](https://github.com/itchyny/mackerel-client-rs/compare/468e1b8..v0.1.0) (2017-10-05)
* initial implementation of the API client of Mackerel
* implement organization API, user APIs, service APIs, role API, monitor APIs, alert APIs,
  dashboard APIs, graph annotation APIs, invitation APIs, and metadata APIs
