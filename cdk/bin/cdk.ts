import { App } from "aws-cdk-lib";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { EcsClusterStack } from "../lib/ecs-cluster-stack";
import { AppClusterStack } from "../lib/app-cluster-stack";
import { LandingLambdaStack } from "../lib/landing-lambda-stack";

const app = new App();
const service = "biyard";

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.biyard.co";
const webDomain = host;
const consoleDomain = `console.${host}`;
const apiDomain = `api.${host}`;
const baseDomain = "biyard.co";
const consoleRepoName = "biyard/console";
const commit = process.env.COMMIT!;

// ECS Cluster (VPC, Cluster, Cloud Map namespace)
const ecsCluster = new EcsClusterStack(
  app,
  `${stackName}-ecs-cluster`,
  {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT,
      region: "ap-northeast-2",
    },
    stage: env,
  },
);

// Console (Dioxus) Fargate + API Gateway
new AppClusterStack(app, `${stackName}-app-cluster`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-app-cluster`,
  baseDomain,
  appDomain: consoleDomain,
  apiDomain,
  repoName: consoleRepoName,
  containerPort: 8080,
  maxCapacity: 20,
  commit,
  vpc: ecsCluster.vpc,
  cluster: ecsCluster.cluster,
  namespace: ecsCluster.namespace,
});

// Landing: Lambda (Dioxus SSR) with Function URL
const landingStack = new LandingLambdaStack(app, `${stackName}-landing-lambda`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-landing-lambda`,
  stage: env,
  commit,
});

// Landing: S3+CloudFront CDN (static assets + Lambda Function URL proxy)
new GlobalAccelStack(app, "landing", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stackName: process.env.WEB_STACK_NAME,
  stage: env,
  commit,

  webDomain,
  baseDomain,
  apiConfig: {
    domain: landingStack.functionUrlDomain,
    prefix: "",
  },
});

// DynamoDB
new GlobalTableStack(app, `${stackName}-dynamodb`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  service,
});
