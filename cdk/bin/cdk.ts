import { App } from "aws-cdk-lib";
import { GlobalAccelStack } from "../lib/global-accel-stack";

const app = new App();

const stackName = process.env.STACK;

const env = process.env.ENV || "dev";
// Common host
const host = process.env.DOMAIN || "dev.biyard.co";
const webDomain = host;
const consoleDomain = `console.${host}`;
const apiDomain = `api.${host}`;
const baseDomain = "biyard.co";

const deployWeb = process.env.DEPLOY_WEB === "true";
const deployConsole = process.env.DEPLOY_CONSOLE === "true";

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

if (deployWeb) {
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
}

if (deployConsole) {
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
}

// new GlobalTableStack(app, `${stackName}-dynamodb`, {
//   env: {
//     account: process.env.CDK_DEFAULT_ACCOUNT,
//     region: "ap-northeast-2",
//   },
// });
