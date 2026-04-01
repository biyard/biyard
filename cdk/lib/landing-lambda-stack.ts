import {
  Stack,
  StackProps,
  aws_lambda as lambda,
  aws_apigatewayv2 as apigw,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_iam as iam,
} from "aws-cdk-lib";
import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import { HttpLambdaIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { Repository } from "aws-cdk-lib/aws-ecr";

export interface LandingLambdaStackProps extends StackProps {
  stage: string;
  commit: string;
  apiDomain: string;
  baseDomain: string;
}

export class LandingLambdaStack extends Stack {
  constructor(scope: Construct, id: string, props: LandingLambdaStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    // HTTP API Gateway
    const httpApi = new apigw.HttpApi(this, "HttpApi", {
      apiName: `biyard-landing-${this.stackName}`,
      description: "Biyard Landing API Gateway",
    });

    // Lambda from ECR Docker image
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

    const lambdaIntegration = new HttpLambdaIntegration(
      "LambdaIntegration",
      landingLambda,
    );

    httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [apigw.HttpMethod.ANY],
      integration: lambdaIntegration,
    });
    httpApi.addRoutes({
      path: "/",
      methods: [apigw.HttpMethod.ANY],
      integration: lambdaIntegration,
    });

    // Certificate for custom domain
    const cert = new acm.Certificate(this, "Cert", {
      domainName: apiDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // Custom domain for API Gateway
    const domainName = new apigw.DomainName(this, "CustomDomain", {
      domainName: apiDomain,
      certificate: cert,
    });

    // API mapping
    new apigw.ApiMapping(this, "ApiMapping", {
      api: httpApi,
      domainName: domainName,
    });

    const region = this.region;

    // Route53 records
    new route53.ARecord(this, "AliasV4", {
      zone,
      recordName: apiDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId,
        ),
      ),
    });
    new route53.AaaaRecord(this, "AliasV6", {
      zone,
      recordName: apiDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId,
        ),
      ),
    });
  }
}
