FROM rust:latest as builder-base

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown && \
    rustup update

RUN cargo install dioxus-cli

FROM builder-base as dioxus

ENTRYPOINT ["dx"]