FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

RUN dx build --release

FROM builder AS debug

EXPOSE 8080

RUN dx serve --release --port 8080