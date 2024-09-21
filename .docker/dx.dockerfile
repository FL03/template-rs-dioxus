FROM rust:latest AS builder-base

RUN rustup default stable && \
    rustup target add wasm32-unknown-unknown && \
    rustup update

FROM builder-base AS builder

RUN cargo install dioxus-cli

FROM debian:stable-slim AS exec-base

RUN apt-get update && \
    apt-get upgrade

RUN rm -rf /var/lib/apt/lists/*

FROM exec-base AS exec

COPY --from=builder /usr/local/cargo/bin/dx /usr/local/bin/dx

ENTRYPOINT ["dx"]