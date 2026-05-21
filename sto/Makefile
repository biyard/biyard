ENV ?= dev
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
PORT ?= 8085

COMMIT ?= $(shell git rev-parse --short HEAD)

RUST_LOG ?= debug
RUSTFLAGS ?= "-D warnings"


AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

REPOSITORY ?= biyard/sto
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY) --query "repositories[0].repositoryUri" --output text)

.PHONY: dev build build-lambda clean fmt clippy docker docker.push docker.login docker.local seed up down

BUILD_ENV ?= DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) RUST_LOG=$(RUST_LOG)

dev:
	$(BUILD_ENV) dx serve --port $(PORT) --web

build:
	$(BUILD_ENV) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features server --platform server

build-lambda:
	$(BUILD_ENV) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features lambda --platform server

docker:
	docker build -t $(ECR):$(COMMIT) -f Dockerfile ../target/dx/sto/release/web

docker.push:
	docker tag $(ECR):$(COMMIT) $(ECR):$(ENV)
	docker push $(ECR):$(COMMIT)
	docker push $(ECR):$(ENV)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

STAGE ?= dev

# DynamoDB 시드 — STAGE=local 이면 LocalStack, dev/prod 면 원격 AWS
seed:
	@STAGE=$(STAGE) bash ../scripts/seed-sto-remote.sh

clean:
	dx clean
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy --features server
	cargo clippy --features web --target wasm32-unknown-unknown
