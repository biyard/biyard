//! Idempotent table bootstrap for the integration test suite.
//!
//! The compile-time `DYNAMO_TABLE_PREFIX` is expected to be `biyard-test`, so
//! the table name is `biyard-test-main`. We create it on demand if missing;
//! subsequent test runs reuse the existing table and rely on UUIDv7 entity
//! ids for isolation.

use aws_sdk_dynamodb::types::{
    AttributeDefinition, BillingMode, GlobalSecondaryIndex, KeySchemaElement, KeyType, Projection,
    ProjectionType, ScalarAttributeType,
};

pub const TABLE_NAME: &str = "biyard-test-main";

pub async fn create_main_table() {
    let cfg = console::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let exists = cli
        .describe_table()
        .table_name(TABLE_NAME)
        .send()
        .await
        .is_ok();
    if exists {
        return;
    }

    let attr = |name: &str| {
        AttributeDefinition::builder()
            .attribute_name(name)
            .attribute_type(ScalarAttributeType::S)
            .build()
            .unwrap()
    };
    let key_hash = |name: &str| {
        KeySchemaElement::builder()
            .attribute_name(name)
            .key_type(KeyType::Hash)
            .build()
            .unwrap()
    };
    let key_range = |name: &str| {
        KeySchemaElement::builder()
            .attribute_name(name)
            .key_type(KeyType::Range)
            .build()
            .unwrap()
    };
    let gsi = |name: &str, pk: &str, sk: &str| {
        GlobalSecondaryIndex::builder()
            .index_name(name)
            .key_schema(key_hash(pk))
            .key_schema(key_range(sk))
            .projection(
                Projection::builder()
                    .projection_type(ProjectionType::All)
                    .build(),
            )
            .build()
            .unwrap()
    };

    cli.create_table()
        .table_name(TABLE_NAME)
        .billing_mode(BillingMode::PayPerRequest)
        .attribute_definitions(attr("pk"))
        .attribute_definitions(attr("sk"))
        .attribute_definitions(attr("gsi1_pk"))
        .attribute_definitions(attr("gsi1_sk"))
        .attribute_definitions(attr("gsi2_pk"))
        .attribute_definitions(attr("gsi2_sk"))
        .attribute_definitions(attr("gsi3_pk"))
        .attribute_definitions(attr("gsi3_sk"))
        .key_schema(key_hash("pk"))
        .key_schema(key_range("sk"))
        .global_secondary_indexes(gsi("gsi1-index", "gsi1_pk", "gsi1_sk"))
        .global_secondary_indexes(gsi("gsi2-index", "gsi2_pk", "gsi2_sk"))
        .global_secondary_indexes(gsi("gsi3-index", "gsi3_pk", "gsi3_sk"))
        .send()
        .await
        .expect("create test table");
}
