ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

ENV ?= dev
STACK ?= biyard-$(ENV)-stack
COMMIT ?= $(shell git rev-parse --short HEAD)

WEB_STACK_NAME=$(STACK)
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

STO_STACK_NAME=$(STACK)-sto-cdn
STO_CDN_ID=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(STO_STACK_NAME) \
  --query "Stacks[0].Outputs[?OutputKey=='CloudFrontDistributionId'].OutputValue" \
  --output text 2>/dev/null)

STO_BUCKET=$(shell aws cloudformation describe-stacks \
  --region us-east-1 \
  --stack-name $(STO_STACK_NAME) \
  --query "Stacks[0].Outputs[?OutputKey=='WebsiteBucket'].OutputValue" \
  --output text 2>/dev/null)

BUILD_CDK_ENV=AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) \
              ENV=$(ENV) STACK=$(STACK) COMMIT=$(COMMIT) \
              WEB_STACK_NAME=$(WEB_STACK_NAME)

infra:
	docker compose --profile infra up -d --remove-orphans

deploy:
	@cd cdk && npm i
	@cd cdk && $(BUILD_CDK_ENV) npm run build
	@cd cdk && $(BUILD_CDK_ENV) cdk synth
	@cd cdk && $(BUILD_CDK_ENV) cdk deploy --require-approval never $(AWS_FLAG) --all --concurrency 5

sync-landing:
	@aws s3 sync target/dx/landing/release/web/public s3://$(WEB_BUCKET) > /dev/null
	@aws cloudfront create-invalidation --distribution-id $(WEB_CDN_ID) --paths "/*" > /dev/null

sync-sto:
	@aws s3 sync target/dx/sto/release/web/public s3://$(STO_BUCKET) > /dev/null
	@aws cloudfront create-invalidation --distribution-id $(STO_CDN_ID) --paths "/*" > /dev/null

seed-sto:
	@AWS_REGION=$(REGION) STAGE=$(ENV) bash scripts/seed-sto-remote.sh
