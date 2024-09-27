FROM scratch AS workspace

WORKDIR /src
ADD . .

FROM node:latest AS node

WORKDIR /app

COPY --from=workspace --chown=755 --link src/input.css .

RUN npm install -g tailwindcss

RUN npx tailwindcss -i ./input.css -o ./tailwind.css --minify

FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

COPY --chown=755 --from=node --link /.docker/tailwind.css /app/public/tailwind.css

RUN dx build --release

FROM nginx:alpine-slim AS runner-base

ENV RUST_LOG=debug

# Copy source files
COPY --chown=755 --from=builder /.docker/dist /usr/share/nginx/html
COPY --chown=755 --from=node /.docker/tailwind.css /usr/share/nginx/html/tailwind.css
# Copy configuration files
COPY --from=workspace --chown=755 --link ./.config/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=workspace --chown=755 --link ./.config/nginx/mime.types /etc/nginx/mime.types
