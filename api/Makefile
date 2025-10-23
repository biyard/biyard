RUST_FLAG ?=
WATCH_TARGET = -w src -w $(PWD)/..

# BUILD_ENV
ENV ?= local
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
DYNAMO_ENDPOINT ?= http://localhost:4566
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
RUST_LOG ?= info

BUILD_ENV ?= ENV=$(ENV) \
						DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
						DYNAMO_ENDPOINT=$(DYNAMO_ENDPOINT) \
						AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) \
						AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) \
						AWS_REGION=$(AWS_REGION) \
						RUST_LOG=$(RUST_LOG) \

run:
	$(BUILD_ENV) cargo watch -x "run $(RUST_FLAG)" $(WATCH_TARGET)

test:
	$(BUILD_ENV) cargo test
