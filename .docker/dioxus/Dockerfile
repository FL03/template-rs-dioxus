FROM rust:latest AS builder-base

RUN rustup default stable && \
    rustup target add wasm32-unknown-unknown && \
    rustup update

FROM builder-base AS builder

RUN cargo install --locked dioxus-cli

ENTRYPOINT ["dx"]

FROM debian:stable-slim AS slim-base

RUN apt-get update -y && \
    apt-get upgrade -y 

RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl-dev \
    pkg-config

RUN rm -rf /var/lib/apt/lists/*

FROM slim-base AS slim

COPY --from=builder /usr/local/cargo/bin/dx /usr/local/bin/dx

ENTRYPOINT ["dx"]