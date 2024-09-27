FROM node:latest AS node

WORKDIR /app

COPY input.css .

RUN npm install -g tailwindcss

RUN npx tailwindcss -i ./input.css -o ./tailwind.css --minify

FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

COPY --chown=755 --from=node --link /app/tailwind.css /app/public/tailwind.css

RUN dx build --release

FROM nginx:alpine-slim AS runner-base

ENV RUST_LOG=debug

# Copy source files
COPY --from=builder --link /app/dist /usr/share/nginx/html
COPY --from=node --link /app/tailwind.css /usr/share/nginx/html/tailwind.css
# Copy configuration files
COPY --from=builder /app/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/nginx/mime.types /etc/nginx/mime.types
