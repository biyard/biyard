import {
  Duration,
  Stack,
  StackProps,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_elasticloadbalancingv2 as elbv2,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_iam as iam,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import { Repository } from "aws-cdk-lib/aws-ecr";
import * as r53Targets from "aws-cdk-lib/aws-route53-targets";

export interface AppClusterStackProps extends StackProps {
  appDomain: string;
  baseDomain: string;
  repoName: string;
  commit: string;
  containerPort?: number;
  healthPath?: string;
  maxCapacity?: number;
}

export class AppClusterStack extends Stack {
  constructor(scope: Construct, id: string, props: AppClusterStackProps) {
    super(scope, id, { ...props });

    const {
      appDomain,
      baseDomain,
      repoName,
      commit,
      containerPort = 8080,
      healthPath = "/version",
      maxCapacity = 20,
    } = props;

    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    const vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });
    const cluster = new ecs.Cluster(this, "Cluster", { vpc });

    // Task execution role
    const taskExecutionRole = new iam.Role(this, "TaskExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
    });
    taskExecutionRole.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AmazonECSTaskExecutionRolePolicy",
      ),
    );

    // ACM Certificate
    const cert = new acm.Certificate(this, "Cert", {
      domainName: appDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // ALB
    const alb = new elbv2.ApplicationLoadBalancer(this, "ALB", {
      vpc,
      internetFacing: true,
    });

    // Task Definition
    const desiredCount = 2;
    const taskDefinition = new ecs.TaskDefinition(this, "TaskDefinition", {
      compatibility: ecs.Compatibility.FARGATE,
      cpu: "256",
      memoryMiB: "512",
      executionRole: taskExecutionRole,
    });

    const repository = Repository.fromRepositoryName(
      this,
      "Repository",
      repoName,
    );

    const container = taskDefinition.addContainer("AppContainer", {
      image: ecs.ContainerImage.fromEcrRepository(repository, commit),
      logging: new ecs.AwsLogDriver({
        streamPrefix: `${this.stackName}-logging`,
      }),
      environment: {
        REGION: this.region,
      },
    });

    container.addPortMappings({
      containerPort,
      protocol: ecs.Protocol.TCP,
    });

    // Fargate Service
    const service = new ecs.FargateService(this, "Service", {
      cluster,
      taskDefinition,
      desiredCount,
      maxHealthyPercent: 200,
      minHealthyPercent: 50,
      assignPublicIp: true,
    });

    const scaling = service.autoScaleTaskCount({
      minCapacity: desiredCount,
      maxCapacity,
    });

    scaling.scaleOnCpuUtilization("CpuScaling", {
      targetUtilizationPercent: 70,
      scaleInCooldown: Duration.seconds(60),
      scaleOutCooldown: Duration.seconds(60),
    });

    // Target Group
    const targetGroup = new elbv2.ApplicationTargetGroup(this, "TargetGroup", {
      targets: [
        service.loadBalancerTarget({
          containerName: "AppContainer",
          containerPort,
        }),
      ],
      protocol: elbv2.ApplicationProtocol.HTTP,
      vpc,
      port: containerPort,
      deregistrationDelay: Duration.seconds(30),
      healthCheck: {
        path: healthPath,
        healthyHttpCodes: "200",
      },
    });

    // HTTPS Listener
    const listener = alb.addListener("HttpsListener", {
      port: 443,
      certificates: [cert],
      open: true,
    });

    listener.addAction("RedirectToHttps", {
      action: elbv2.ListenerAction.redirect({ protocol: "HTTPS", port: "443" }),
    });
    listener.addTargetGroups("TargetGroupRule", {
      targetGroups: [targetGroup],
    });

    // Route53 records: app.dev.biyard.co → ALB
    const recordName = appDomain.replace(`.${baseDomain}`, "");
    new route53.ARecord(this, "AlbAliasV4", {
      zone,
      recordName,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.LoadBalancerTarget(alb),
      ),
    });
    new route53.AaaaRecord(this, "AlbAliasV6", {
      zone,
      recordName,
      target: route53.RecordTarget.fromAlias(
        new r53Targets.LoadBalancerTarget(alb),
      ),
    });
  }
}
