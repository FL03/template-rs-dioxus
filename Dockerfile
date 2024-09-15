FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

RUN dx build --release

FROM builder as debug

RUN dx serve

FROM alpine:latest

COPY --from=builder /app/dist/ /app/


