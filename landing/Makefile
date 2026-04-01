PORT ?= 8001

.PHONY: dev build clean fmt clippy

dev:
	dx serve --port $(PORT) --web

build:
	dx build --release --platform web

clean:
	dx clean
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy --target wasm32-unknown-unknown
