FROM rust:latest AS builder-base

RUN rustup target add wasm32-unknown-unknown && \
    rustup update

FROM builder-base AS builder

RUN cargo install --locked dioxus-cli

FROM debian:stable-slim AS exec-base

RUN apt-get update && \
    apt-get upgrade

RUN rm -rf /var/lib/apt/lists/*

FROM exec-base AS exec

COPY --from=builder /usr/local/cargo/bin/dx /usr/local/bin/dx

ENTRYPOINT ["dx"]