import { App } from "aws-cdk-lib";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { AppClusterStack } from "../lib/app-cluster-stack";

const app = new App();
const service = "biyard";

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.biyard.co";
const webDomain = host;
const appDomain = `app.${host}`;
const baseDomain = "biyard.co";
const appRepoName = "biyard/app";
const commit = process.env.COMMIT!;

// App (Dioxus) Fargate cluster
new AppClusterStack(app, `${stackName}-app-cluster`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-app-cluster`,
  baseDomain,
  appDomain,
  repoName: appRepoName,
  containerPort: 8080,
  maxCapacity: 20,
  healthPath: "/version",
  commit,
});

// Landing: pure S3+CloudFront CDN (no API proxy)
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
