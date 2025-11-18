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
import { Construct } from "constructs";
import { RegionalClusterStack } from "./regional-cluster-stack";

export interface RegionalServiceStackProps extends StackProps {
  repoName: string;
  commit: string;
  cluster: RegionalClusterStack;

  healthPath?: string;
  maxCapacity?: number;
  containerPort?: number;
}

export class RegionalServiceStack extends Stack {
  constructor(scope: Construct, id: string, props: RegionalServiceStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const {
      commit,
      repoName,
      containerPort = 3000,
      maxCapacity = 50,
      healthPath = "/health",
    } = props;
    const desiredCount = 2;
    const maxHealthyPercent = 200;
    const minHealthyPercent = 50;
    const memoryMiB = "512";
    const cpu = "256";

    const { vpc, cluster, taskExecutionRole, listener } = props.cluster;

    const taskDefinition = new ecs.TaskDefinition(this, "TaskDefinition", {
      compatibility: ecs.Compatibility.FARGATE,
      cpu,
      memoryMiB,
      executionRole: taskExecutionRole,
    });

    const repository = Repository.fromRepositoryName(
      this,
      "Repository",
      repoName,
    );
    const container = taskDefinition.addContainer(
      `${this.stackName}-Container`,
      {
        image: ecs.ContainerImage.fromEcrRepository(repository, commit),
        logging: new ecs.AwsLogDriver({
          streamPrefix: `${this.stackName}-logging`,
        }),
        environment: {
          REGION: this.region,
        },
      },
    );

    container.addPortMappings({
      containerPort,
      protocol: ecs.Protocol.TCP,
    });

    const service = new ecs.FargateService(this, "Service", {
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

    const targetGroup = new elbv2.ApplicationTargetGroup(this, "TargetGroup", {
      targets: [
        service.loadBalancerTarget({
          containerName: `${this.stackName}-Container`,
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

    listener.addTargetGroups("TgRuleApiHost", {
      targetGroups: [targetGroup],
    });
  }
}
