FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as trunkrs

RUN cargo install trunk wasm-bindgen-cli

FROM trunkrs as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN trunk --config Trunk.toml build --release

FROM nginx:latest as production

COPY --from=builder /workspace/dist /usr/share/nginx/html
