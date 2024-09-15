FROM jo3mccain/dioxus:latest AS builder-base

WORKDIR /app
ADD . .

RUN npm install -g tailwindcss

FROM builder-base AS builder

RUN npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify
RUN dx build --release

FROM builder AS debug

EXPOSE 8080

RUN dx serve --release --port 8080