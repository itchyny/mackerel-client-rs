use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::collections::HashMap;
use strum::{Display, EnumString};
use typed_builder::TypedBuilder;

use crate::client::*;
use crate::entity::{Entity, Id};
use crate::error::Result;
use crate::role::RoleFullname;

/// An AWS integration entity
pub type AWSIntegration = Entity<AWSIntegrationValue>;

/// An AWS integration id
pub type AWSIntegrationId = Id<AWSIntegrationValue>;

/// An AWS integration value
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AWSIntegrationValue {
    pub name: String,
    #[builder(default)]
    pub memo: String,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default)]
    pub region: String,
    #[builder(default)]
    #[serde(default)]
    pub included_tags: String,
    #[builder(default)]
    #[serde(default)]
    pub excluded_tags: String,
    #[builder(default)]
    #[serde(default)]
    pub services: HashMap<AWSServiceName, AWSServiceConfig>,
}

/// An AWS integration service config
#[derive(PartialEq, Clone, Debug, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
#[serde(rename_all = "camelCase")]
pub struct AWSServiceConfig {
    #[builder(default = true)]
    pub enable: bool,
    #[builder(default, setter(strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<RoleFullname>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_metrics: Vec<String>,
    #[builder(default)]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub retire_automatically: bool,
}

/// AWS service name
#[derive(
    PartialEq, Eq, Clone, Hash, Debug, Display, EnumString, SerializeDisplay, DeserializeFromStr,
)]
pub enum AWSServiceName {
    EC2,
    ELB,
    ALB,
    NLB,
    RDS,
    Redshift,
    ElastiCache,
    SQS,
    Lambda,
    DynamoDB,
    CloudFront,
    APIGateway,
    Kinesis,
    S3,
    ES,
    ECSCluster,
    SES,
    States,
    EFS,
    Firehose,
    Batch,
    WAF,
    Billing,
    Route53,
    Connect,
    DocDB,
    CodeBuild,
    #[strum(default)]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde_json::json;

    fn aws_integration_example1() -> AWSIntegration {
        AWSIntegration::builder()
            .id("aws1")
            .value(
                AWSIntegrationValue::builder()
                    .name("AWS integration setting")
                    .key("aws-access-key")
                    .region("ap-northeast-1")
                    .build(),
            )
            .build()
    }

    fn json_example1() -> serde_json::Value {
        json!({
            "id": "aws1",
            "name": "AWS integration setting",
            "memo": "",
            "key": "aws-access-key",
            "region": "ap-northeast-1",
            "includedTags": "",
            "excludedTags": "",
            "services": {},
        })
    }

    fn aws_integration_example2() -> AWSIntegration {
        AWSIntegration::builder()
            .id("aws2")
            .value(
                AWSIntegrationValue::builder()
                    .name("AWS integration setting")
                    .memo("This is an AWS integration memo.")
                    .role_arn("aws-role-arn")
                    .external_id("aws-integration-external-id")
                    .region("ap-northeast-2")
                    .included_tags("tag1, tag2")
                    .excluded_tags("tag3, tag4")
                    .services([
                        (
                            AWSServiceName::EC2,
                            AWSServiceConfig::builder()
                                .retire_automatically(true)
                                .build(),
                        ),
                        (
                            AWSServiceName::NLB,
                            AWSServiceConfig::builder().role("aws:nlb").build(),
                        ),
                        (
                            AWSServiceName::S3,
                            AWSServiceConfig::builder()
                                .enable(false)
                                .role("aws:s3")
                                .excluded_metrics(["s3.bucket_size.*".to_owned()])
                                .build(),
                        ),
                    ])
                    .build(),
            )
            .build()
    }

    fn json_example2() -> serde_json::Value {
        json!({
            "id": "aws2",
            "name": "AWS integration setting",
            "memo": "This is an AWS integration memo.",
            "roleArn": "aws-role-arn",
            "externalId": "aws-integration-external-id",
            "region": "ap-northeast-2",
            "includedTags": "tag1, tag2",
            "excludedTags": "tag3, tag4",
            "services": {
                "EC2": {
                    "enable": true,
                    "retireAutomatically": true,
                },
                "NLB": {
                    "enable": true,
                    "role": "aws:nlb",
                },
                "S3": {
                    "enable": false,
                    "role": "aws:s3",
                    "excludedMetrics": ["s3.bucket_size.*"],
                },
            },
        })
    }

    #[rstest]
    #[case(aws_integration_example1(), json_example1())]
    #[case(aws_integration_example2(), json_example2())]
    fn test_aws_integration_json(
        #[case] aws_integration: AWSIntegration,
        #[case] json: serde_json::Value,
    ) {
        assert_eq!(serde_json::to_value(&aws_integration).unwrap(), json);
        assert_eq!(aws_integration, serde_json::from_value(json).unwrap());
    }

    #[rstest]
    #[case(AWSServiceName::EC2, "EC2")]
    #[case(AWSServiceName::ELB, "ELB")]
    #[case(AWSServiceName::ALB, "ALB")]
    #[case(AWSServiceName::NLB, "NLB")]
    #[case(AWSServiceName::RDS, "RDS")]
    #[case(AWSServiceName::Redshift, "Redshift")]
    #[case(AWSServiceName::ElastiCache, "ElastiCache")]
    #[case(AWSServiceName::SQS, "SQS")]
    #[case(AWSServiceName::Lambda, "Lambda")]
    #[case(AWSServiceName::DynamoDB, "DynamoDB")]
    #[case(AWSServiceName::CloudFront, "CloudFront")]
    #[case(AWSServiceName::APIGateway, "APIGateway")]
    #[case(AWSServiceName::Kinesis, "Kinesis")]
    #[case(AWSServiceName::S3, "S3")]
    #[case(AWSServiceName::ES, "ES")]
    #[case(AWSServiceName::ECSCluster, "ECSCluster")]
    #[case(AWSServiceName::SES, "SES")]
    #[case(AWSServiceName::States, "States")]
    #[case(AWSServiceName::EFS, "EFS")]
    #[case(AWSServiceName::Firehose, "Firehose")]
    #[case(AWSServiceName::Batch, "Batch")]
    #[case(AWSServiceName::WAF, "WAF")]
    #[case(AWSServiceName::Billing, "Billing")]
    #[case(AWSServiceName::Route53, "Route53")]
    #[case(AWSServiceName::Connect, "Connect")]
    #[case(AWSServiceName::DocDB, "DocDB")]
    #[case(AWSServiceName::CodeBuild, "CodeBuild")]
    fn test_aws_service(#[case] aws_service: AWSServiceName, #[case] aws_service_str: &str) {
        assert_eq!(aws_service.to_string(), aws_service_str);
        assert_eq!(aws_service, aws_service_str.parse().unwrap());
        assert_eq!(
            aws_service,
            serde_json::from_value(aws_service_str.into()).unwrap()
        );
        assert_eq!(serde_json::to_value(aws_service).unwrap(), aws_service_str);
    }
}

impl Client {
    /// Fetches all the AWS integration settings.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#list>.
    pub async fn list_aws_integrations(&self) -> Result<Vec<AWSIntegration>> {
        self.request(
            Method::GET,
            "/api/v0/aws-integrations",
            query_params![],
            request_body![],
            response_body! { aws_integrations: Vec<AWSIntegration> },
        )
        .await
    }

    /// Creates a new AWS integration.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#create>.
    pub async fn create_aws_integration(
        &self,
        aws_integration_value: &AWSIntegrationValue,
    ) -> Result<AWSIntegration> {
        self.request(
            Method::POST,
            "/api/v0/aws-integrations",
            query_params![],
            request_body!(aws_integration_value),
            response_body!(..),
        )
        .await
    }

    /// Gets an AWS integration.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#get>.
    pub async fn get_aws_integration(
        &self,
        aws_integration_id: impl Into<AWSIntegrationId>,
    ) -> Result<AWSIntegration> {
        self.request(
            Method::GET,
            format_url!("/api/v0/aws-integrations/{}", aws_integration_id),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Updates an AWS integration.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#update>.
    pub async fn update_aws_integration(
        &self,
        aws_integration_id: impl Into<AWSIntegrationId>,
        aws_integration_value: &AWSIntegrationValue,
    ) -> Result<AWSIntegration> {
        self.request(
            Method::PUT,
            format_url!("/api/v0/aws-integrations/{}", aws_integration_id),
            query_params![],
            request_body!(aws_integration_value),
            response_body!(..),
        )
        .await
    }

    /// Deletes an AWS integration.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#delete>.
    pub async fn delete_aws_integration(
        &self,
        aws_integration_id: impl Into<AWSIntegrationId>,
    ) -> Result<AWSIntegration> {
        self.request(
            Method::DELETE,
            format_url!("/api/v0/aws-integrations/{}", aws_integration_id),
            query_params![],
            request_body![],
            response_body!(..),
        )
        .await
    }

    /// Generates AWS integration external ID.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#generate-external-id>.
    pub async fn create_aws_integration_external_id(&self) -> Result<String> {
        self.request(
            Method::POST,
            "/api/v0/aws-integrations-external-id",
            query_params![],
            request_body![],
            response_body! { externalId: String },
        )
        .await
    }

    /// Lists excludable metrics.
    ///
    /// See <https://mackerel.io/api-docs/entry/aws-integration#excludable-metrics>.
    pub async fn list_aws_integration_excludable_metrics(
        &self,
    ) -> Result<HashMap<AWSServiceName, Vec<String>>> {
        self.request(
            Method::GET,
            "/api/v0/aws-integrations-excludable-metrics",
            query_params![],
            request_body![],
            |response: HashMap<AWSServiceName, Vec<String>>| response,
        )
        .await
    }
}

#[cfg(test)]
mod client_tests {
    use serde_json::json;

    use crate::aws_integration::*;
    use crate::tests::*;

    fn value_example() -> AWSIntegrationValue {
        AWSIntegrationValue::builder()
            .name("AWS integration setting")
            .memo("This is an AWS integration memo.")
            .role_arn("aws-role-arn")
            .external_id("aws-integration-external-id")
            .region("ap-northeast-1")
            .services([(
                AWSServiceName::EC2,
                AWSServiceConfig::builder()
                    .role("aws: ec2")
                    .retire_automatically(true)
                    .build(),
            )])
            .build()
    }

    fn entity_example() -> AWSIntegration {
        AWSIntegration {
            id: AWSIntegrationId::from("aws0"),
            value: value_example(),
        }
    }

    fn value_json_example() -> serde_json::Value {
        json!({
            "name": "AWS integration setting",
            "memo": "This is an AWS integration memo.",
            "roleArn": "aws-role-arn",
            "externalId": "aws-integration-external-id",
            "region": "ap-northeast-1",
            "includedTags": "",
            "excludedTags": "",
            "services": {
                "EC2": {
                    "enable": true,
                    "role": "aws:ec2",
                    "retireAutomatically": true,
                },
            },
        })
    }

    fn entity_json_example() -> serde_json::Value {
        let mut json = value_json_example();
        json["id"] = json!("aws0");
        json
    }

    #[async_std::test]
    async fn list_aws_integrations() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/aws-integrations",
            response = json!({
                "aws_integrations": [entity_json_example()],
            }),
        };
        assert_eq!(
            test_client!(server).list_aws_integrations().await,
            Ok(vec![entity_example()]),
        );
    }

    #[async_std::test]
    async fn create_aws_integration() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/aws-integrations",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .create_aws_integration(&value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn get_aws_integration() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/aws-integrations/aws0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).get_aws_integration("aws0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .get_aws_integration(AWSIntegrationId::from("aws0"))
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn update_aws_integration() {
        let server = test_server! {
            method = PUT,
            path = "/api/v0/aws-integrations/aws0",
            request = value_json_example(),
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server)
                .update_aws_integration("aws0", &value_example())
                .await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .update_aws_integration(AWSIntegrationId::from("aws0"), &value_example())
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn delete_aws_integration() {
        let server = test_server! {
            method = DELETE,
            path = "/api/v0/aws-integrations/aws0",
            response = entity_json_example(),
        };
        assert_eq!(
            test_client!(server).delete_aws_integration("aws0").await,
            Ok(entity_example()),
        );
        assert_eq!(
            test_client!(server)
                .delete_aws_integration(AWSIntegrationId::from("aws0"))
                .await,
            Ok(entity_example()),
        );
    }

    #[async_std::test]
    async fn create_aws_integration_external_id() {
        let server = test_server! {
            method = POST,
            path = "/api/v0/aws-integrations-external-id",
            response = json!({ "externalId": "external0" }),
        };
        assert_eq!(
            test_client!(server)
                .create_aws_integration_external_id()
                .await,
            Ok("external0".to_owned())
        );
    }

    #[async_std::test]
    async fn list_aws_integration_excludable_metrics() {
        let server = test_server! {
            method = GET,
            path = "/api/v0/aws-integrations-excludable-metrics",
            response = json!({
                "EC2": ["ec2.cpu.used"],
                "ELB": ["elb.count.request_count"],
                "ALB": ["alb.request.count"],
                "NLB": ["nlb.flowcount.active"],
                "RDS": ["rds.cpu.used"],
                "Redshift": ["redshift.cpu.used"],
                "ElastiCache": ["elasticache.cpu.used"],
                "SQS": ["sqs.oldest_message.age"],
                "Lambda": ["lambda.count.invocations"],
                "DynamoDB": ["dynamodb.read_capacity_units.provisioned"],
                "CloudFront": ["cloudfront.requests.request"],
                "APIGateway": ["apigateway.requests.count"],
                "Kinesis": ["kinesis.bytes.get_records"],
                "S3": ["s3.requests.all_requests"],
                "ES": ["es.cluster_status.green"],
                "ECSCluster": ["ecs.cpu_utilization.maximum"],
                "SES": ["ses.email_sending_events.send"],
                "States": ["states.executions.aborted"],
                "EFS": ["efs.burst_credit_balance.minimum"],
                "Firehose": ["firehose.service_bytes.backup_to_s3"],
                "Batch": ["batch.job_queue_status.#.succeeded"],
                "WAF": ["waf.web_acl_requests.#.allowed"],
                "Billing": ["billing.estimated_charges.total"],
                "Route53": ["route53.dns_queries.*"],
                "Connect": ["connect.voice_calls.breaching_concurrency_quota"],
                "DocDB": ["docdb.cpu.used"],
                "CodeBuild": ["codebuild.builds.count"],
                "UnknownService": [],
            }),
        };
        assert_eq!(
            test_client!(server)
                .list_aws_integration_excludable_metrics()
                .await
                .map(|_| ()),
            Ok(())
        );
    }
}
