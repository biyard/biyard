ENV ?= dev
DYNAMO_TABLE_PREFIX ?= biyard-$(ENV)
PORT ?= 8000

.PHONY: dev build build-lambda clean fmt clippy test

dev:
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) dx serve --port $(PORT) --web

build:
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) dx build --release \
		@client --features web --platform web \
		@server --features server --platform server

build-lambda:
	DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) dx build --release \
		@client --features web --platform web \
		@server --features lambda --platform server

clean:
	dx clean
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy --features server
	cargo clippy --features web --target wasm32-unknown-unknown

test:
	cargo test --features server
