FROM node:latest AS node

WORKDIR /app

COPY input.css .

RUN npm install -g tailwindcss

RUN npx tailwindcss -i ./input.css -o ./tailwind.css --minify

FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

COPY --from=node /app/tailwind.css /app/public/tailwind.css

RUN dx build --release

FROM nginx AS runner-base

ENV RUST_LOG=debug

COPY --from=builder --chmod=755 /app/dist /usr/share/nginx/html
COPY --from=node --chmod=755 /app/tailwind.css /usr/share/nginx/html/tailwind.css
COPY --from=builder /app/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/nginx/mime.types /etc/nginx/conf/mime.types
