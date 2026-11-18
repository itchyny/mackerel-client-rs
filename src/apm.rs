use chrono::{DateTime, Utc};
use http::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::error::Result;
use crate::log::OrderDirection;

/// Parameters to list HTTP server statistics
#[derive(PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[non_exhaustive]
pub struct ListHTTPServerStatsParams {
    pub service_name: String,
    #[builder(default, setter(strip_option))]
    pub service_namespace: Option<String>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    #[builder(default, setter(strip_option))]
    pub environment: Option<String>,
    #[builder(default, setter(strip_option))]
    pub version: Option<String>,
    #[builder(default, setter(strip_option))]
    pub method: Option<String>,
    #[builder(default, setter(strip_option))]
    pub route: Option<String>,
    #[builder(default, setter(strip_option))]
    pub order_column: Option<HTTPServerStatsOrderColumn>,
    #[builder(default, setter(strip_option))]
    pub order_direction: Option<OrderDirection>,
    #[builder(default, setter(!into, strip_option))]
    pub page: Option<u32>,
    #[builder(default, setter(!into, strip_option))]
    pub per_page: Option<u8>,
}

/// Column to sort HTTP server statistics by
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum HTTPServerStatsOrderColumn {
    Sum,
    Average,
    P95,
    ErrorRate,
    RequestCount,
}

/// HTTP server statistics of a route
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct HTTPServerStats {
    pub method: String,
    pub route: String,
    #[builder(setter(!into))]
    pub total_millis: f64,
    #[builder(setter(!into))]
    pub average_millis: f64,
    #[builder(setter(!into))]
    pub approx_p95_millis: f64,
    #[builder(setter(!into))]
    pub error_rate_percentage: f64,
    #[builder(setter(!into))]
    pub request_count: u64,
}

/// Parameters to list database query statistics
#[derive(PartialEq, Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
#[non_exhaustive]
pub struct ListDBQueryStatsParams {
    pub service_name: String,
    #[builder(default, setter(strip_option))]
    pub service_namespace: Option<String>,
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    #[builder(default, setter(strip_option))]
    pub environment: Option<String>,
    #[builder(default, setter(strip_option))]
    pub version: Option<String>,
    #[builder(default, setter(strip_option))]
    pub query: Option<String>,
    #[builder(default, setter(strip_option))]
    pub order_column: Option<DBQueryStatsOrderColumn>,
    #[builder(default, setter(strip_option))]
    pub order_direction: Option<OrderDirection>,
    #[builder(default, setter(!into, strip_option))]
    pub page: Option<u32>,
    #[builder(default, setter(!into, strip_option))]
    pub per_page: Option<u8>,
}

/// Column to sort database query statistics by
#[derive(
    PartialEq, Eq, Copy, Clone, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum DBQueryStatsOrderColumn {
    Sum,
    Average,
    P95,
    ExecutionCount,
}

/// Database query statistics of a query
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct DBQueryStats {
    pub query: String,
    #[builder(setter(!into))]
    pub execution_count: u64,
    #[builder(setter(!into))]
    pub total_millis: f64,
    #[builder(setter(!into))]
    pub average_millis: f64,
    #[builder(setter(!into))]
    pub approx_p95_millis: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    #[test]
    fn test_http_server_stats_json() {
        let http_server_stats = HTTPServerStats::builder()
            .method("GET")
            .route("/api/users")
            .total_millis(837.0)
            .average_millis(9.01)
            .approx_p95_millis(19.89)
            .error_rate_percentage(0.5)
            .request_count(93)
            .build();
        let json = json!({
            "method": "GET",
            "route": "/api/users",
            "totalMillis": 837.0,
            "averageMillis": 9.01,
            "approxP95Millis": 19.89,
            "errorRatePercentage": 0.5,
            "requestCount": 93,
        });
        assert_eq!(serde_json::to_value(&http_server_stats).unwrap(), json);
        assert_eq!(http_server_stats, serde_json::from_value(json).unwrap());
    }

    #[test]
    fn test_db_query_stats_json() {
        let db_query_stats = DBQueryStats::builder()
            .query("SELECT * FROM users WHERE id = ?")
            .execution_count(1250)
            .total_millis(11262.5)
            .average_millis(9.01)
            .approx_p95_millis(19.89)
            .build();
        let json = json!({
            "query": "SELECT * FROM users WHERE id = ?",
            "executionCount": 1250,
            "totalMillis": 11262.5,
            "averageMillis": 9.01,
            "approxP95Millis": 19.89,
        });
        assert_eq!(serde_json::to_value(&db_query_stats).unwrap(), json);
        assert_eq!(db_query_stats, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(HTTPServerStatsOrderColumn::Sum, "SUM")]
    #[case(HTTPServerStatsOrderColumn::Average, "AVERAGE")]
    #[case(HTTPServerStatsOrderColumn::P95, "P95")]
    #[case(HTTPServerStatsOrderColumn::ErrorRate, "ERROR_RATE")]
    #[case(HTTPServerStatsOrderColumn::RequestCount, "REQUEST_COUNT")]
    fn test_http_server_stats_order_column(
        #[case] order_column: HTTPServerStatsOrderColumn,
        #[case] order_column_str: &str,
    ) {
        assert_eq!(order_column.to_string(), order_column_str);
        assert_eq!(order_column, order_column_str.parse().unwrap());
    }

    #[rstest]
    #[case(DBQueryStatsOrderColumn::Sum, "SUM")]
    #[case(DBQueryStatsOrderColumn::Average, "AVERAGE")]
    #[case(DBQueryStatsOrderColumn::P95, "P95")]
    #[case(DBQueryStatsOrderColumn::ExecutionCount, "EXECUTION_COUNT")]
    fn test_db_query_stats_order_column(
        #[case] order_column: DBQueryStatsOrderColumn,
        #[case] order_column_str: &str,
    ) {
        assert_eq!(order_column.to_string(), order_column_str);
        assert_eq!(order_column, order_column_str.parse().unwrap());
    }
}

impl Client {
    /// Fetches HTTP server statistics.
    ///
    /// The range between `from` and `to` must be 30 minutes or less.
    ///
    /// See <https://mackerel.io/api-docs/entry/apm#http-server-stats>.
    pub async fn list_http_server_stats(
        &self,
        params: impl Borrow<ListHTTPServerStatsParams>,
    ) -> Result<(Vec<HTTPServerStats>, bool)> {
        let params = params.borrow();
        self.request(
            Method::GET,
            "/api/v0/apm/http-server-stats",
            query_params! {
                serviceName = params.service_name,
                serviceNamespace = params.service_namespace.as_deref().unwrap_or_default(),
                from = params.from.timestamp().to_string(),
                to = params.to.timestamp().to_string(),
                environment = params.environment.as_deref().unwrap_or_default(),
                version = params.version.as_deref().unwrap_or_default(),
                method = params.method.as_deref().unwrap_or_default(),
                route = params.route.as_deref().unwrap_or_default(),
                orderColumn = params.order_column.map(|column| column.to_string()).unwrap_or_default(),
                orderDirection = params.order_direction.map(|direction| direction.to_string()).unwrap_or_default(),
                page = params.page.map(|page| page.to_string()).unwrap_or_default(),
                perPage = params.per_page.map(|per_page| per_page.to_string()).unwrap_or_default(),
            },
            request_body![],
            response_body! { results: Vec<HTTPServerStats>, hasNextPage: bool },
        )
        .await
    }

    /// Fetches database query statistics.
    ///
    /// The range between `from` and `to` must be 30 minutes or less.
    ///
    /// See <https://mackerel.io/api-docs/entry/apm#db-query-stats>.
    pub async fn list_db_query_stats(
        &self,
        params: impl Borrow<ListDBQueryStatsParams>,
    ) -> Result<(Vec<DBQueryStats>, bool)> {
        let params = params.borrow();
        self.request(
            Method::GET,
            "/api/v0/apm/db-query-stats",
            query_params! {
                serviceName = params.service_name,
                serviceNamespace = params.service_namespace.as_deref().unwrap_or_default(),
                from = params.from.timestamp().to_string(),
                to = params.to.timestamp().to_string(),
                environment = params.environment.as_deref().unwrap_or_default(),
                version = params.version.as_deref().unwrap_or_default(),
                query = params.query.as_deref().unwrap_or_default(),
                orderColumn = params.order_column.map(|column| column.to_string()).unwrap_or_default(),
                orderDirection = params.order_direction.map(|direction| direction.to_string()).unwrap_or_default(),
                page = params.page.map(|page| page.to_string()).unwrap_or_default(),
                perPage = params.per_page.map(|per_page| per_page.to_string()).unwrap_or_default(),
            },
            request_body![],
            response_body! { results: Vec<DBQueryStats>, hasNextPage: bool },
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use crate::apm::*;
    use crate::tests::*;

    #[tokio::test]
    async fn list_http_server_stats() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/apm/http-server-stats",
            query_params = "serviceName=shoppingcart&from=1718802000&to=1718803800",
            response = json!({
                "results": [{
                    "method": "GET",
                    "route": "/api/users",
                    "totalMillis": 837.0,
                    "averageMillis": 9.01,
                    "approxP95Millis": 19.89,
                    "errorRatePercentage": 0.0,
                    "requestCount": 93,
                }],
                "hasNextPage": false,
            }),
        };
        let params = ListHTTPServerStatsParams::builder()
            .service_name("shoppingcart")
            .from(DateTime::from_timestamp(1718802000, 0).unwrap())
            .to(DateTime::from_timestamp(1718803800, 0).unwrap())
            .build();
        let expected = Ok((
            vec![HTTPServerStats::builder()
                .method("GET")
                .route("/api/users")
                .total_millis(837.0)
                .average_millis(9.01)
                .approx_p95_millis(19.89)
                .error_rate_percentage(0.0)
                .request_count(93)
                .build()],
            false,
        ));
        assert_eq!(
            test_client!(server).list_http_server_stats(&params).await,
            expected,
        );
        assert_eq!(
            test_client!(server).list_http_server_stats(params).await,
            expected,
        );
    }

    #[tokio::test]
    async fn list_http_server_stats_with_options() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/apm/http-server-stats",
            query_params = "serviceName=shoppingcart&serviceNamespace=shop&from=1718802000&to=1718803800&\
                            environment=production&version=1.0.0&method=GET&route=/api/users&\
                            orderColumn=ERROR_RATE&orderDirection=ASC&page=2&perPage=50",
            response = json!({ "results": [], "hasNextPage": true }),
        };
        assert_eq!(
            test_client!(server)
                .list_http_server_stats(
                    ListHTTPServerStatsParams::builder()
                        .service_name("shoppingcart")
                        .service_namespace("shop")
                        .from(DateTime::from_timestamp(1718802000, 0).unwrap())
                        .to(DateTime::from_timestamp(1718803800, 0).unwrap())
                        .environment("production")
                        .version("1.0.0")
                        .method("GET")
                        .route("/api/users")
                        .order_column(HTTPServerStatsOrderColumn::ErrorRate)
                        .order_direction(OrderDirection::Asc)
                        .page(2)
                        .per_page(50)
                        .build(),
                )
                .await,
            Ok((vec![], true)),
        );
    }

    #[tokio::test]
    async fn list_db_query_stats() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/apm/db-query-stats",
            query_params = "serviceName=shoppingcart&from=1718802000&to=1718803800&\
                            query=SELECT&orderColumn=EXECUTION_COUNT&orderDirection=DESC",
            response = json!({
                "results": [{
                    "query": "SELECT * FROM users WHERE id = ?",
                    "executionCount": 1250,
                    "totalMillis": 11262.5,
                    "averageMillis": 9.01,
                    "approxP95Millis": 19.89,
                }],
                "hasNextPage": false,
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_db_query_stats(
                    ListDBQueryStatsParams::builder()
                        .service_name("shoppingcart")
                        .from(DateTime::from_timestamp(1718802000, 0).unwrap())
                        .to(DateTime::from_timestamp(1718803800, 0).unwrap())
                        .query("SELECT")
                        .order_column(DBQueryStatsOrderColumn::ExecutionCount)
                        .order_direction(OrderDirection::Desc)
                        .build(),
                )
                .await,
            Ok((
                vec![DBQueryStats::builder()
                    .query("SELECT * FROM users WHERE id = ?")
                    .execution_count(1250)
                    .total_millis(11262.5)
                    .average_millis(9.01)
                    .approx_p95_millis(19.89)
                    .build()],
                false,
            )),
        );
    }
}
