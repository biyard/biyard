PROJECT ?= biyard
SERVICE ?= api
COMMIT ?= $(shell git rev-parse --short HEAD)

RUST_FLAG ?=
WEB_BUILD ?= true
WATCH_TARGET = -w src -w $(PWD)/
ifeq ($(WEB_BUILD),false)
	WATCH_TARGET += -w ../console -w ../landing
endif

# BUILD_ENV
ENV ?= local
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
DYNAMO_ENDPOINT ?= http://localhost:4566
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
RUST_LOG ?= info
DOMAIN ?= dev.biyard.co

REPO_NAME ?= $(PROJECT)/$(SERVICE)
ECR_NAME ?= $(shell aws ecr describe-repositories --repository-names $(REPO_NAME)  --query "repositories[0].repositoryUri" --output text)

CONSOLE_BASE_PATH ?= /console
LANDING_BASE_PATH ?= /landing

BUILD_ENV ?= ENV=$(ENV) \
						DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
						DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
						AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
						AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
						AWS_REGION=$(AWS_REGION) \
						RUST_LOG=$(RUST_LOG) \
						SLACK_HOOK=$(SLACK_HOOK) \
						DOMAIN=$(DOMAIN) \
						CONSOLE_BASE_PATH=$(CONSOLE_BASE_PATH) \
						LANDING_BASE_PATH=$(LANDING_BASE_PATH)

run:
	$(BUILD_ENV) cargo watch -x "run $(RUST_FLAG)" $(WATCH_TARGET)

test:
	$(BUILD_ENV) cargo test

docker.build:
	@docker run --rm --name $(SERVICE) -v $(PWD)/..:/app -w /app/$(SERVICE) rust:slim-bookworm sh -c 'apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && $(BUILD_ENV) CARGO_TARGET_DIR=/app/$(SERVICE)/target cargo build --release $(RUST_FLAG)'

	@docker build -t $(ECR_NAME):$(COMMIT) --no-cache -f Dockerfile $(PWD)/target/release

docker.push:
	@docker push $(ECR_NAME):$(COMMIT)

docker.login:
	@aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com