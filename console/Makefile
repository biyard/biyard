ENV ?= dev
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
DYNAMODB_ENDPOINT ?= none
PORT ?= 8080

COMMIT ?= $(shell git rev-parse --short HEAD)

AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)

REPOSITORY ?= biyard/console
ECR ?= $(shell aws ecr describe-repositories --repository-names $(REPOSITORY) --query "repositories[0].repositoryUri" --output text)

.PHONY: dev build build-testing build-lambda clean fmt clippy test docker docker.push docker.login tailwind


dev: 
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
	DYNAMODB_ENDPOINT=$(DYNAMODB_ENDPOINT) \
	AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
	AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
	AWS_REGION=$(AWS_REGION) \
	dx serve --port $(PORT) --web

build:
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features server --platform server

build-testing:
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) dx build --release --debug-symbols false \
		@client --features web --platform web \
		@server --features server --platform server

build-lambda:
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) dx build --release --debug-symbols false \
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

test:
	mkdir -p ./test-build-target/debug/deps/public
	npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
	$(BUILD_ENV) CARGO_TARGET_DIR=./test-build-target cargo test --features server --tests