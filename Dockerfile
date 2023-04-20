FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly && \
    cargo install trunk wasm-bindgen-cli

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN trunk build
RUN cargo build -p xtask --release

FROM builder

EXPOSE 8080

ENTRYPOINT [ "trunk" ]
CMD [ "serve" ]

FROM scratch as wasm

COPY --from=builder /workspace/dist /app/dist
COPY --from=builder /workspace/target/release/xtask /app/xtask

WORKDIR /app

EXPOSE 8080
