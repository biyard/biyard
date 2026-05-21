import * as cdk from "aws-cdk-lib";
import { Stack, StackProps, RemovalPolicy } from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";

export class GlobalTableStack extends Stack {
  constructor(
    scope: Construct,
    id: string,
    props: StackProps & { service: string; stage: string },
  ) {
    super(scope, id, props);

    const { stage, service } = props;

    const ddb = new dynamodb.Table(this, "GlobalTable", {
      tableName: `${service}-${stage}-main`,
      partitionKey: { name: "pk", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sk", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      pointInTimeRecoverySpecification: {
        pointInTimeRecoveryEnabled: true,
      },
      removalPolicy: RemovalPolicy.RETAIN,
      replicationRegions: ["eu-central-1", "us-east-1"],
      deletionProtection: true,
    });
    ddb.addGlobalSecondaryIndex({
      indexName: "type-index",
      projectionType: dynamodb.ProjectionType.ALL,
      partitionKey: {
        name: "sk",
        type: dynamodb.AttributeType.STRING,
      },
    });

    for (const i of [1, 2, 3, 4, 5, 6]) {
      ddb.addGlobalSecondaryIndex({
        indexName: `gsi${i}-index`,
        projectionType: dynamodb.ProjectionType.ALL,
        partitionKey: {
          name: `gsi${i}_pk`,
          type: dynamodb.AttributeType.STRING,
        },
        sortKey: { name: `gsi${i}_sk`, type: dynamodb.AttributeType.STRING },
      });
    }

    new cdk.CfnOutput(this, "DDBTableName", {
      value: ddb.tableName,
    });

    const stoTable = new dynamodb.Table(this, "StoTable", {
      tableName: `${service}-${stage}-sto`,
      partitionKey: { name: "pk", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sk", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      pointInTimeRecoverySpecification: {
        pointInTimeRecoveryEnabled: true,
      },
      removalPolicy: RemovalPolicy.RETAIN,
      deletionProtection: true,
    });
    stoTable.addGlobalSecondaryIndex({
      indexName: "type-index",
      projectionType: dynamodb.ProjectionType.ALL,
      partitionKey: { name: "sk", type: dynamodb.AttributeType.STRING },
    });
    for (const i of [1, 2, 3, 4, 5, 6]) {
      stoTable.addGlobalSecondaryIndex({
        indexName: `gsi${i}-index`,
        projectionType: dynamodb.ProjectionType.ALL,
        partitionKey: {
          name: `gsi${i}_pk`,
          type: dynamodb.AttributeType.STRING,
        },
        sortKey: { name: `gsi${i}_sk`, type: dynamodb.AttributeType.STRING },
      });
    }

    new cdk.CfnOutput(this, "StoTableName", {
      value: stoTable.tableName,
    });
  }
}
