FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install dx CLI
RUN cargo install dioxus-cli --locked

# wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY sto/ sto/

# Build (DYNAMO_TABLE_PREFIX required at compile time)
ARG DYNAMO_TABLE_PREFIX=biyard-local
ENV DYNAMO_TABLE_PREFIX=${DYNAMO_TABLE_PREFIX}

RUN cd sto && dx build --release --debug-symbols false \
    @client --features web --platform web \
    @server --features server --platform server

# ── runtime ──────────────────────────────────────────────────────────────────
FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /build/target/dx/sto/release/web/ .

EXPOSE 8085

CMD ["./sto"]
