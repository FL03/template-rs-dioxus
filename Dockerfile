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

FROM nginx AS runner

ENV RUST_LOG=debug

COPY --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/nginx/nginx.conf /etc/nginx/nginx.conf
