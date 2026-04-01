ENV ?= dev
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
PORT ?= 8001

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

REPOSITORY ?= biyard/landing
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY) --query "repositories[0].repositoryUri" --output text)

.PHONY: dev build build-lambda clean fmt clippy docker docker.push docker.login


dev:
	dx serve --port $(PORT) --web

build:
	dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features server --platform server

build-lambda:
	dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features lambda --platform server

docker:
	docker build -t $(ECR):$(COMMIT) -f Dockerfile ../target/dx/landing/release/web

docker.push:
	docker tag $(ECR):$(COMMIT) $(ECR):$(ENV)
	docker push $(ECR):$(COMMIT)
	docker push $(ECR):$(ENV)

docker.login:
	aws ecr get-login-password --region $(AWS_REGION) | docker login --username AWS --password-stdin $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com

clean:
	dx clean
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy --features server
	cargo clippy --features web --target wasm32-unknown-unknown
