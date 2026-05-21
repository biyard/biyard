import {
  Stack,
  StackProps,
  aws_lambda as lambda,
  aws_iam as iam,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { Repository } from "aws-cdk-lib/aws-ecr";

export interface StoLambdaStackProps extends StackProps {
  stage: string;
  service: string;
  commit: string;
}

export class StoLambdaStack extends Stack {
  public readonly functionUrlDomain: string;

  constructor(scope: Construct, id: string, props: StoLambdaStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { stage, service, commit } = props;

    const repository = Repository.fromRepositoryName(
      this,
      "StoRepository",
      "biyard/sto",
    );

    const stoLambda = new lambda.DockerImageFunction(this, "Function", {
      code: lambda.DockerImageCode.fromEcr(repository, {
        tagOrDigest: commit,
      }),
      environment: {
        REGION: this.region,
        DISABLE_ANSI: "true",
        NO_COLOR: "true",
        IP: "0.0.0.0",
        PORT: "8080",
        DYNAMO_TABLE_PREFIX: `${service}-${stage}`,
      },
      memorySize: 512,
      timeout: cdk.Duration.seconds(30),
    });

    stoLambda.addToRolePolicy(
      new iam.PolicyStatement({
        actions: [
          "dynamodb:GetItem",
          "dynamodb:BatchGetItem",
          "dynamodb:Query",
          "dynamodb:Scan",
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
          "dynamodb:BatchWriteItem",
          "dynamodb:TransactGetItems",
          "dynamodb:TransactWriteItems",
        ],
        resources: [
          `arn:aws:dynamodb:${this.region}:${this.account}:table/${service}-${stage}-sto`,
          `arn:aws:dynamodb:${this.region}:${this.account}:table/${service}-${stage}-sto/index/*`,
        ],
      }),
    );

    const functionUrl = stoLambda.addFunctionUrl({
      authType: lambda.FunctionUrlAuthType.NONE,
    });

    this.functionUrlDomain = cdk.Fn.select(2, cdk.Fn.split("/", functionUrl.url));

    new cdk.CfnOutput(this, "FunctionUrl", {
      value: functionUrl.url,
    });
  }
}
