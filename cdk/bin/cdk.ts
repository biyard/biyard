import { App } from "aws-cdk-lib";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";
import { RegionalClusterStack } from "../lib/regional-cluster-stack";
import cluster from "cluster";
import { RegionalServiceStack } from "../lib/regional-service-stack";

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

const clusterAp = new RegionalClusterStack(app, `${stackName}-cluster`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stackName: `${stackName}-cluster`,
  baseDomain,
  apiDomain,
});

const serviceAp = new RegionalServiceStack(
  app,
  `${stackName}-api-ap-northeast-2`,
  {
    env: {
      account: process.env.CDK_DEFAULT_ACCOUNT,
      region: "ap-northeast-2",
    },
    stackName: `${stackName}-api-ap-northeast-2`,
    cluster: clusterAp,
    repoName: apiRepoName,
    commit,

    containerPort: 3000,
    maxCapacity: 20,
    healthPath: "/version",
  },
);

serviceAp.addDependency(clusterAp);

new GlobalAccelStack(app, "GlobalAccel", {
  stackName,
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stage: env,
  commit: process.env.COMMIT!,

  webDomain,
  apiDomain,
  baseDomain,
});

new GlobalAccelStack(app, "Console", {
  stackName: `${stackName}-console`,
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "us-east-1",
  },
  stage: env,
  commit: process.env.COMMIT!,

  webDomain: consoleDomain,
  apiDomain,
  baseDomain,
});

new GlobalTableStack(app, `${stackName}-dynamodb`, {
  env: {
    account: process.env.CDK_DEFAULT_ACCOUNT,
    region: "ap-northeast-2",
  },
  stage: env,
  service,
});
