import {
  CfnOutput,
  Duration,
  Stack,
  StackProps,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_elasticloadbalancingv2 as elbv2,
  aws_apigatewayv2 as apigw,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_iam as iam,
  aws_servicediscovery as sd,
  aws_logs as logs,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import { Repository } from "aws-cdk-lib/aws-ecr";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";
import { HttpAlbIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";

export interface AppClusterStackProps extends StackProps {
  appDomain: string;
  apiDomain?: string;
  baseDomain: string;
  repoName: string;
  commit: string;
  containerPort?: number;
  maxCapacity?: number;

  vpc: ec2.IVpc;
  cluster: ecs.ICluster;
  namespace: sd.PrivateDnsNamespace;
}

export class AppClusterStack extends Stack {
  constructor(scope: Construct, id: string, props: AppClusterStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const {
      appDomain,
      apiDomain,
      baseDomain,
      repoName,
      commit,
      containerPort = 8080,
      maxCapacity = 20,
      vpc,
      cluster,
      namespace,
    } = props;

    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    // --- HTTP API ---
    const httpApi = new apigw.HttpApi(this, "HttpApi", {
      apiName: `biyard-console-${this.stackName}`,
      description: "Biyard Console API Gateway",
    });

    // --- ECS Fargate ---
    const repository = Repository.fromRepositoryName(
      this,
      "Repository",
      repoName,
    );

    const albSg = new ec2.SecurityGroup(this, "AlbSG", {
      vpc,
      description: "Internal ALB security group",
      allowAllOutbound: true,
    });
    albSg.addIngressRule(
      ec2.Peer.ipv4(vpc.vpcCidrBlock),
      ec2.Port.tcp(80),
      "Internal HTTP from VPC",
    );

    const sg = new ec2.SecurityGroup(this, "AppSG", {
      vpc,
      description: "Console ECS security group",
      allowAllOutbound: true,
    });
    sg.addIngressRule(
      albSg,
      ec2.Port.tcp(containerPort),
      "From internal ALB",
    );
    sg.addIngressRule(
      ec2.Peer.ipv4(vpc.vpcCidrBlock),
      ec2.Port.tcp(containerPort),
      "Direct from VPC (CloudMap A record callers)",
    );

    const taskExecutionRole = new iam.Role(this, "TaskExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
    });
    taskExecutionRole.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AmazonECSTaskExecutionRolePolicy",
      ),
    );

    const taskDefinition = new ecs.TaskDefinition(this, "TaskDefinition", {
      compatibility: ecs.Compatibility.FARGATE,
      cpu: "256",
      memoryMiB: "512",
      executionRole: taskExecutionRole,
    });

    const container = taskDefinition.addContainer("AppContainer", {
      image: ecs.ContainerImage.fromEcrRepository(repository, commit),
      logging: new ecs.AwsLogDriver({
        streamPrefix: `${this.stackName}-logging`,
        logRetention: logs.RetentionDays.TWO_WEEKS,
      }),
      environment: {
        REGION: this.region,
        IP: "0.0.0.0",
        PORT: String(containerPort),
      },
    });

    container.addPortMappings({
      containerPort,
      protocol: ecs.Protocol.TCP,
    });

    const desiredCount = 2;
    const fargateService = new ecs.FargateService(this, "Service", {
      cluster,
      taskDefinition,
      desiredCount,
      maxHealthyPercent: 200,
      minHealthyPercent: 100,
      assignPublicIp: true,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
      securityGroups: [sg],
      cloudMapOptions: {
        name: "api",
        cloudMapNamespace: namespace,
        // A record (MULTIVALUE by default) — callable as
        // `http://api.biyard-<stage>-svc.local:<containerPort>` from
        // same-VPC clients (e.g. ratel app-shell Lambda). API Gateway
        // reaches the service via ALB, not this DNS entry.
        dnsRecordType: sd.DnsRecordType.A,
        container,
        containerPort,
      },
    });

    const scaling = fargateService.autoScaleTaskCount({
      minCapacity: desiredCount,
      maxCapacity,
    });

    scaling.scaleOnCpuUtilization("CpuScaling", {
      targetUtilizationPercent: 70,
      scaleInCooldown: Duration.seconds(60),
      scaleOutCooldown: Duration.seconds(60),
    });

    // --- Internal ALB in front of Fargate ---
    // Shared by:
    //   - API Gateway HTTP API (public `api.<host>`) via HttpAlbIntegration
    //   - (optional) direct in-VPC callers that want load balancing +
    //     health checks instead of DNS-based MULTIVALUE.
    const supportedSubnets = vpc.publicSubnets.filter(
      (s) => s.availabilityZone !== "ap-northeast-2d",
    );

    // NOTE: this VPC only has public subnets, so we place the ALB in public
    // subnets. `internetFacing: true` is required for public-subnet ALBs;
    // actual access is restricted to the VPC CIDR via `albSg` ingress rules,
    // so the ALB is effectively internal-only.
    const alb = new elbv2.ApplicationLoadBalancer(this, "InternalAlb", {
      vpc,
      internetFacing: true,
      vpcSubnets: { subnets: supportedSubnets },
      securityGroup: albSg,
    });

    const listener = alb.addListener("HttpListener", {
      port: 80,
      protocol: elbv2.ApplicationProtocol.HTTP,
    });

    listener.addTargets("FargateTg", {
      port: containerPort,
      protocol: elbv2.ApplicationProtocol.HTTP,
      targets: [fargateService],
      deregistrationDelay: Duration.seconds(20),
      healthCheck: {
        path: "/v1/health",
        healthyHttpCodes: "200",
        interval: Duration.seconds(15),
        healthyThresholdCount: 2,
        unhealthyThresholdCount: 3,
      },
    });

    // --- API Gateway integration via ALB ---
    // Explicit VpcLink: auto-created VpcLinks default to private subnets,
    // which this VPC does not have.
    const vpcLink = new apigw.VpcLink(this, "VpcLink", {
      vpc,
      subnets: { subnets: supportedSubnets },
      securityGroups: [albSg],
    });

    const albIntegration = new HttpAlbIntegration(
      "EcsAlbIntegration",
      listener,
      { vpcLink },
    );

    httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [apigw.HttpMethod.ANY],
      integration: albIntegration,
    });
    httpApi.addRoutes({
      path: "/",
      methods: [apigw.HttpMethod.ANY],
      integration: albIntegration,
    });

    new CfnOutput(this, "InternalApiDnsName", {
      value: `api.${namespace.namespaceName}`,
      exportName: `${this.stackName}-InternalApiDnsName`,
      description:
        "Private CloudMap DNS for in-VPC consumers (e.g. ratel app-shell Lambda)",
    });
    new CfnOutput(this, "InternalApiPort", {
      value: String(containerPort),
      exportName: `${this.stackName}-InternalApiPort`,
      description: "Container port to use with InternalApiDnsName",
    });

    // --- Custom Domain + Route53 ---
    const cert = new acm.Certificate(this, "Cert", {
      domainName: appDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    const domainName = new apigw.DomainName(this, "CustomDomain", {
      domainName: appDomain,
      certificate: cert,
    });

    new apigw.ApiMapping(this, "ApiMapping", {
      api: httpApi,
      domainName,
    });

    const recordName = appDomain.replace(`.${baseDomain}`, "");
    new route53.ARecord(this, "AliasV4", {
      zone,
      recordName,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId,
        ),
      ),
    });
    new route53.AaaaRecord(this, "AliasV6", {
      zone,
      recordName,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.ApiGatewayv2DomainProperties(
          domainName.regionalDomainName,
          domainName.regionalHostedZoneId,
        ),
      ),
    });

    // --- API Domain (optional) ---
    if (apiDomain) {
      const apiCert = new acm.Certificate(this, "ApiCert", {
        domainName: apiDomain,
        validation: acm.CertificateValidation.fromDns(zone),
      });

      const apiDomainName = new apigw.DomainName(this, "ApiCustomDomain", {
        domainName: apiDomain,
        certificate: apiCert,
      });

      new apigw.ApiMapping(this, "ApiDomainMapping", {
        api: httpApi,
        domainName: apiDomainName,
      });

      const apiRecordName = apiDomain.replace(`.${baseDomain}`, "");
      new route53.ARecord(this, "ApiAliasV4", {
        zone,
        recordName: apiRecordName,
        target: route53.RecordTarget.fromAlias(
          new r53Targets.ApiGatewayv2DomainProperties(
            apiDomainName.regionalDomainName,
            apiDomainName.regionalHostedZoneId,
          ),
        ),
      });
      new route53.AaaaRecord(this, "ApiAliasV6", {
        zone,
        recordName: apiRecordName,
        target: route53.RecordTarget.fromAlias(
          new r53Targets.ApiGatewayv2DomainProperties(
            apiDomainName.regionalDomainName,
            apiDomainName.regionalHostedZoneId,
          ),
        ),
      });
    }

  }
}
