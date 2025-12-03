ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

ENV ?= dev
STACK ?= biyard-$(ENV)-stack
COMMIT ?= $(shell git rev-parse --short HEAD)

WEB_STACK_NAME=$(STACK)-landing
WEB_CDN_ID=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(WEB_STACK_NAME) \
  --query "Stacks[0].Outputs[?OutputKey=='CloudFrontDistributionId'].OutputValue" \
  --output text)

WEB_BUCKET=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(WEB_STACK_NAME) \
  --query "Stacks[0].Outputs[?OutputKey=='WebsiteBucket'].OutputValue" \
  --output text)

CONSOLE_STACK_NAME=$(STACK)-console
CONSOLE_CDN_ID=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(CONSOLE_STACK_NAME) \
  --query "Stacks[0].Outputs[?OutputKey=='CloudFrontDistributionId'].OutputValue" \
  --output text)

CONSOLE_BUCKET=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(CONSOLE_STACK_NAME) \
  --query "Stacks[0].Outputs[?OutputKey=='WebsiteBucket'].OutputValue" \
  --output text)

BUILD_CDK_ENV=AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) \
              ENV=$(ENV) STACK=$(STACK) COMMIT=$(COMMIT) \
              WEB_STACK_NAME=$(WEB_STACK_NAME) CONSOLE_STACK_NAME=$(CONSOLE_STACK_NAME)

deploy:
	@cd cdk && npm i
	@cd cdk && $(BUILD_CDK_ENV) npm run build
	@cd cdk && $(BUILD_CDK_ENV) cdk synth
	@cd cdk && $(BUILD_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG) --all --concurrency 5

sync-landing: clean landing/dist
	@aws s3 sync landing/dist s3://$(WEB_BUCKET) > /dev/null
	@aws cloudfront create-invalidation --distribution-id $(WEB_CDN_ID) --paths "/*" > /dev/null

sync-console: clean console/dist
	@aws s3 sync console/dist s3://$(CONSOLE_BUCKET) > /dev/null
	@aws cloudfront create-invalidation --distribution-id $(CONSOLE_CDN_ID) --paths "/*" > /dev/null

clean:
	rm -rf landing/dist
	rm -rf console/dist

%/dist:
	cd $* && make build
