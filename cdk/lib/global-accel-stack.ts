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
}

export class GlobalAccelStack extends Stack {
  constructor(scope: Construct, id: string, props: GlobalAccelStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

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
      },
    );
    const spaRoutingFunction = new cloudfront.Function(
      this,
      `SpaRoutingFunction`,
      {
        code: cloudfront.FunctionCode.fromInline(`
    function handler(event) {
      var request = event.request;
      var uri = request.uri;

      // Check if URI has a file extension
      var hasFileExtension = /\\.[a-zA-Z0-9]+$/.test(uri);

      // If no file extension, it's a page route - append index.html
      if (!hasFileExtension) {
        // Special handling for /admin route
        if (uri === '/admin' || uri === '/admin/') {
          request.uri = '/admin/index.html';
        }
        // For other routes without trailing slash, add /index.html
        else if (!uri.endsWith('/')) {
          request.uri = uri + '/index.html';
        }
        // For routes with trailing slash, add index.html
        else {
          request.uri = uri + 'index.html';
        }
      }

      return request;
    }
          `),
      },
    );

    // CloudFront cert (must be in us-east-1). Use provided ARN or create DNS‑validated one.
    const cachedS3Prop = {
      origin: s3Origin,
      cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
      compress: true,
    };

    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin: s3Origin,
        cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        functionAssociations: [
          {
            function: spaRoutingFunction,
            eventType: cloudfront.FunctionEventType.VIEWER_REQUEST,
          },
        ],
      },
      defaultRootObject: "index.html",
      additionalBehaviors: {
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
    });

    // ---- Route53 alias for the end-user domain → CloudFront ----
    new route53.ARecord(this, "AliasV4", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""), // e.g., 'dev'
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(distribution),
      ),
    });
    new route53.AaaaRecord(this, "AliasV6", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(distribution),
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
