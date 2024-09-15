FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

RUN dx build --release



