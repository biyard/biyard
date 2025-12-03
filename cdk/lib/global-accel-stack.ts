import {
  Stack,
  StackProps,
  aws_route53 as route53,
  aws_certificatemanager as acm,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as cdk from "aws-cdk-lib";
import * as targets from "aws-cdk-lib/aws-route53-targets";
import * as s3 from "aws-cdk-lib/aws-s3";

export interface GlobalAccelStackProps extends StackProps {
  commit: string;
  stage: string;

  webDomain: string;
  apiDomain: string;
  baseDomain: string;

  // ALB DNS for SSR origin (optional - if not provided, defaults to S3-only)
  albDnsName?: string;
}

export class GlobalAccelStack extends Stack {
  constructor(scope: Construct, id: string, props: GlobalAccelStackProps) {
    super(scope, id, { ...props });

    const { webDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    const cert = new acm.Certificate(this, "Cert", {
      domainName: webDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // 1) S3 for static assets
    const staticBucket = new s3.Bucket(this, "StaticBucket", {
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    const oai = new cloudfront.OriginAccessIdentity(this, "OAI");
    staticBucket.grantRead(oai);

    const s3Origin = origins.S3BucketOrigin.withOriginAccessIdentity(
      staticBucket,
      {
        originAccessIdentity: oai,
      }
    );

    // 2) ALB origin for SSR (if provided)
    const albOrigin = props.albDnsName
      ? new origins.HttpOrigin(props.albDnsName, {
          protocolPolicy: cloudfront.OriginProtocolPolicy.HTTPS_ONLY,
          httpsPort: 443,
          originSslProtocols: [cloudfront.OriginSslPolicy.TLS_V1_2],
          readTimeout: cdk.Duration.seconds(60),
          keepaliveTimeout: cdk.Duration.seconds(5),
        })
      : undefined;

    // 2.5) CloudFront Function for domain-based routing
    const domainRoutingFunction = new cloudfront.Function(
      this,
      "DomainRoutingFunction",
      {
        code: cloudfront.FunctionCode.fromInline(`
function handler(event) {
  var request = event.request;
  var host = request.headers.host.value;
  var uri = request.uri;

  // Static files pass through unchanged
  var hasFileExtension = /\\.[a-zA-Z0-9]+$/.test(uri);
  if (hasFileExtension) {
    return request;
  }

  // Already has /landing or /console prefix
  if (uri.startsWith('/landing') || uri.startsWith('/console')) {
    return request;
  }

  // Console domain -> add /console prefix
  if (host.startsWith('console.')) {
    request.uri = '/console' + uri;
  }
  // Landing domain (default) -> add /landing prefix
  else {
    request.uri = '/landing' + uri;
  }

  return request;
}
        `),
      }
    );

    // 3) Cache policies
    const noCachePolicy = new cloudfront.CachePolicy(this, "NoCachePolicy", {
      cachePolicyName: `${id}-NoCache`,
      minTtl: cdk.Duration.seconds(0),
      maxTtl: cdk.Duration.seconds(0),
      defaultTtl: cdk.Duration.seconds(0),
      enableAcceptEncodingGzip: true,
      enableAcceptEncodingBrotli: true,
      headerBehavior: cloudfront.CacheHeaderBehavior.allowList(
        "Authorization",
        "Cookie",
        "Host"
      ),
      queryStringBehavior: cloudfront.CacheQueryStringBehavior.all(),
      cookieBehavior: cloudfront.CacheCookieBehavior.all(),
    });

    const originRequestPolicy = new cloudfront.OriginRequestPolicy(
      this,
      "OriginRequestPolicy",
      {
        originRequestPolicyName: `${id}-ForwardAll`,
        headerBehavior: cloudfront.OriginRequestHeaderBehavior.allViewer(),
        queryStringBehavior: cloudfront.OriginRequestQueryStringBehavior.all(),
        cookieBehavior: cloudfront.OriginRequestCookieBehavior.all(),
      }
    );

    // CloudFront cert (must be in us-east-1). Use provided ARN or create DNS‑validated one.
    const cachedS3Prop = {
      origin: s3Origin,
      cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
      compress: true,
    };

    // Determine default behavior based on whether ALB is provided
    const defaultBehavior = albOrigin
      ? {
          origin: albOrigin,
          cachePolicy: noCachePolicy,
          originRequestPolicy: originRequestPolicy,
          viewerProtocolPolicy:
            cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
          allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
          compress: true,
          functionAssociations: [
            {
              function: domainRoutingFunction,
              eventType: cloudfront.FunctionEventType.VIEWER_REQUEST,
            },
          ],
        }
      : {
          origin: s3Origin,
          cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
          viewerProtocolPolicy:
            cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        };

    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior,
      additionalBehaviors: {
        // App-specific paths (S3 CDN)
        "/landing/*": cachedS3Prop,
        "/console/*": cachedS3Prop,

        // Legacy paths (keep for backward compatibility)
        "/metadata/*": cachedS3Prop,
        "/assets/*": cachedS3Prop,
        "/icons/*": cachedS3Prop,
        "/images/*": cachedS3Prop,
        "/public/*": cachedS3Prop,
        "/animations/*": cachedS3Prop,
        "/documents/*": cachedS3Prop,
        "/logos/*": cachedS3Prop,
        "/sounds/*": cachedS3Prop,
        "/videos/*": cachedS3Prop,
        "/*.js": cachedS3Prop,
        "/*.css": cachedS3Prop,
        "/*.html": cachedS3Prop,
        "/*.ico": cachedS3Prop,
        "/*.svg": cachedS3Prop,
        "/*.avif": cachedS3Prop,
        "/*.png": cachedS3Prop,
        "/*.wasm": cachedS3Prop,
      },

      domainNames: [webDomain],
      certificate: cert,
      httpVersion: cloudfront.HttpVersion.HTTP2_AND_3,
      priceClass: cloudfront.PriceClass.PRICE_CLASS_ALL,

      // Error responses for SPA routing (when using ALB)
      // When ALB returns 404, CloudFront will let the ALB handle it (for SPA fallback)
      errorResponses: albOrigin
        ? [
            {
              httpStatus: 404,
              responseHttpStatus: 404,
              ttl: cdk.Duration.seconds(0),
            },
            {
              httpStatus: 403,
              responseHttpStatus: 403,
              ttl: cdk.Duration.seconds(0),
            },
          ]
        : undefined,
    });

    // ---- Route53 alias for the end-user domain → CloudFront ----
    new route53.ARecord(this, "AliasV4", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""), // e.g., 'dev'
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(distribution)
      ),
    });
    new route53.AaaaRecord(this, "AliasV6", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(distribution)
      ),
    });

    new cdk.CfnOutput(this, "CloudFrontDistributionId", {
      value: distribution.distributionId,
    });

    new cdk.CfnOutput(this, "WebsiteBucket", {
      value: staticBucket.bucketName,
    });
  }
}
