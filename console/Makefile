ENV ?= dev
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
DYNAMO_ENDPOINT ?= none
API_DOMAIN ?= api.$(ENV).biyard.co
PORT ?= 8080

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
DEPLOYER_PRIVATE_KEY ?=

REPOSITORY ?= biyard/console
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY) --query "repositories[0].repositoryUri" --output text)

RUSTFLAGS ?= -D warnings

.PHONY: dev build build-testing build-lambda clean fmt clippy test docker docker.push docker.login tailwind

BUILD_ENV ?= DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
	DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
	API_DOMAIN=$(API_DOMAIN) \
	AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
	AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
	AWS_REGION=$(AWS_REGION) \
	RUSTFLAGS='$(RUSTFLAGS)' \
	DEPLOYER_PRIVATE_KEY=$(DEPLOYER_PRIVATE_KEY)


dev: 
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --port $(PORT) --web

build:
	$(BUILD_ENV) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features server --platform server

build-testing:
	$(BUILD_ENV) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features server --platform server

build-lambda:
	$(BUILD_ENV) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features lambda --platform server

docker:
	docker build -t $(ECR):$(COMMIT) -f Dockerfile ../target/dx/console/release/web

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

test: DYNAMO_TABLE_PREFIX = biyard-test
test: DYNAMO_ENDPOINT = http://localhost:4566
test: AWS_ACCESS_KEY_ID = test
test: AWS_SECRET_ACCESS_KEY = test
test: AWS_REGION = us-east-1
test:
	mkdir -p ./test-build-target/debug/deps/public
	npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
	$(BUILD_ENV) CARGO_TARGET_DIR=./test-build-target cargo test --features server --tests