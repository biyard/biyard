import { App } from "aws-cdk-lib";
import { RegionalServiceStack } from "../lib/regional-service-stack";
import { GlobalAccelStack } from "../lib/global-accel-stack";
import { GlobalTableStack } from "../lib/dynamodb-stack";

const app = new App();

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.biyard.co";
const webDomain = host;
const apiDomain = `api.${host}`;
const baseDomain = "biyard.co";

// new RegionalServiceStack(app, `${stackName}-ap-northeast-2`, {
//   env: {
//     account: process.env.CDK_DEFAULT_ACCOUNT,
//     region: "ap-northeast-2",
//   },
//   fullDomainName: host,
//   healthCheckPath: "/version",
//   commit: process.env.COMMIT!,
//   pghost: process.env.PGHOST_AP!,
//   enableDaemon: true,
//   baseDomain,
//   apiDomain,
// });

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

// new GlobalTableStack(app, `${stackName}-dynamodb`, {
//   env: {
//     account: process.env.CDK_DEFAULT_ACCOUNT,
//     region: "ap-northeast-2",
//   },
// });
