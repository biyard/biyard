import {
  Duration,
  Stack,
  StackProps,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_ecs_patterns as ecs_patterns,
  aws_elasticloadbalancingv2 as elbv2,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_cloudfront as cloudfront,
  aws_cloudfront_origins as origins,
  aws_route53_targets as targets,
  aws_iam as iam,
} from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { RegionalClusterStack } from "./regional-cluster-stack";

export interface RegionalServiceStackProps {
  repoName: string;
  commit: string;

  healthPath?: string;
  maxCapacity?: number;
  containerPort?: number;
}

export class RegionalServiceStack {
  constructor(scope: RegionalClusterStack, props: RegionalServiceStackProps) {
    const {
      commit,
      repoName,
      containerPort = 3000,
      maxCapacity = 50,
      healthPath = "/health",
    } = props;
    const { vpc, cluster, alb, cert, taskExecutionRole } = scope;

    const desiredCount = 2;
    const maxHealthyPercent = 200;
    const minHealthyPercent = 50;
    const memoryMiB = "512";
    const cpu = "256";

    // Create task execution role in the service stack to avoid cross-stack dependencies
    const taskDefinition = new ecs.TaskDefinition(scope, "TaskDefinition", {
      compatibility: ecs.Compatibility.FARGATE,
      cpu,
      memoryMiB,
      executionRole: taskExecutionRole,
    });

    const repository = Repository.fromRepositoryName(
      scope,
      "Repository",
      repoName,
    );
    const container = taskDefinition.addContainer(
      `${scope.stackName}-Container`,
      {
        image: ecs.ContainerImage.fromEcrRepository(repository, commit),
        logging: new ecs.AwsLogDriver({
          streamPrefix: `${scope.stackName}-logging`,
        }),
        environment: {
          REGION: scope.region,
        },
      },
    );

    container.addPortMappings({
      containerPort,
      protocol: ecs.Protocol.TCP,
    });

    const service = new ecs.FargateService(scope, "Service", {
      cluster,
      taskDefinition: taskDefinition,
      desiredCount,
      maxHealthyPercent,
      minHealthyPercent,
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

    const targetGroup = new elbv2.ApplicationTargetGroup(scope, "TargetGroup", {
      targets: [
        service.loadBalancerTarget({
          containerName: `${scope.stackName}-Container`,
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

    // Create a listener rule instead of modifying the listener's default action
    // Scope avoids circular dependency between cluster and service stacks
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
  }
}
