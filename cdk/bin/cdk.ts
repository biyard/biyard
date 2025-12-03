import { App } from "aws-cdk-lib";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { RegionalClusterStack } from "../lib/regional-cluster-stack";

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
const apiRepoName = "biyard/api";
const commit = process.env.COMMIT!;
new RegionalClusterStack(app, `${stackName}-cluster`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-cluster`,
  baseDomain,
  apiDomain,

  apiServiceProps: {
    repoName: apiRepoName,
    containerPort: 3000,
    maxCapacity: 20,
    healthPath: "/version",

    commit,
  },
});

new GlobalAccelStack(app, "landing", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stackName: process.env.WEB_STACK_NAME,
  stage: env,
  commit: process.env.COMMIT!,

  webDomain,
  baseDomain,
  apiConfig: {
    domain: apiDomain,
    prefix: "/landing",
  },
});

new GlobalAccelStack(app, "console", {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stackName: process.env.CONSOLE_STACK_NAME,
  stage: env,
  commit: process.env.COMMIT!,

  webDomain: consoleDomain,
  baseDomain,
  apiConfig: {
    domain: apiDomain,
    prefix: "/console",
  },
});

new GlobalTableStack(app, `${stackName}-dynamodb`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  service,
});
