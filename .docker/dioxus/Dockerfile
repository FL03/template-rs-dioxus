FROM rust:latest AS builder-base

RUN rustup default stable && \
    rustup target add wasm32-unknown-unknown && \
    rustup update

FROM builder-base AS builder

RUN cargo install --locked dioxus-cli

FROM builder AS rsdx

ENTRYPOINT ["dx"]
