import { App } from "aws-cdk-lib";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
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
const landingApiDomain = `landing-api.${host}`;
const baseDomain = "biyard.co";
const consoleRepoName = "biyard/console";
const commit = process.env.COMMIT!;

// Console (Dioxus) Fargate cluster
new AppClusterStack(app, `${stackName}-app-cluster`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-app-cluster`,
  baseDomain,
  appDomain: consoleDomain,
  repoName: consoleRepoName,
  containerPort: 8080,
  maxCapacity: 20,
  healthPath: "/version",
  commit,
});

// Landing: Lambda (Dioxus SSR)
new LandingLambdaStack(app, `${stackName}-landing-lambda`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-landing-lambda`,
  stage: env,
  commit,
  baseDomain,
  apiDomain: landingApiDomain,
});

// Landing: S3+CloudFront CDN (static assets + API Gateway proxy)
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
    domain: landingApiDomain,
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
