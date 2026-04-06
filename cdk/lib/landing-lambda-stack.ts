import {
  Stack,
  StackProps,
  aws_lambda as lambda,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import { Repository } from "aws-cdk-lib/aws-ecr";

export interface LandingLambdaStackProps extends StackProps {
  stage: string;
  commit: string;
}

export class LandingLambdaStack extends Stack {
  public readonly functionUrlDomain: string;

  constructor(scope: Construct, id: string, props: LandingLambdaStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const repository = Repository.fromRepositoryName(
      this,
      "LandingRepository",
      "biyard/landing",
    );

    const landingLambda = new lambda.DockerImageFunction(this, "Function", {
      code: lambda.DockerImageCode.fromEcr(repository, {
        tagOrDigest: props.commit,
      }),
      environment: {
        REGION: this.region,
        DISABLE_ANSI: "true",
        NO_COLOR: "true",
        IP: "0.0.0.0",
        PORT: "8080",
      },
      memorySize: 128,
      timeout: cdk.Duration.seconds(30),
    });

    const functionUrl = landingLambda.addFunctionUrl({
      authType: lambda.FunctionUrlAuthType.NONE,
    });

    // Extract domain from Function URL (https://xxx.lambda-url.region.on.aws/)
    this.functionUrlDomain = cdk.Fn.select(2, cdk.Fn.split("/", functionUrl.url));

    new cdk.CfnOutput(this, "FunctionUrl", {
      value: functionUrl.url,
    });
  }
}
