FROM rust:latest AS builder-base

RUN rustup default stable && \
    rustup target add wasm32-unknown-unknown && \
    rustup update

FROM builder-base AS builder

RUN cargo install --locked dioxus-cli wasm-bindgen-cli


FROM debian:stable-slim AS slim-base

ARG GID=1000 \
    UID=1000 \
    GROUP=runners \
    USERNAME=runner

RUN groupadd -g ${GID} ${GROUP} && \
    useradd -g ${GROUP} -m -u ${UID} ${USERNAME}

# Switch to the custom user
USER ${USERNAME}

RUN apt-get update -y && \
    apt-get upgrade -y

RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libssl-dev \
    pkg-config
