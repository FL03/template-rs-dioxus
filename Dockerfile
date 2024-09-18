FROM node:latest AS node

WORKDIR /app

COPY input.css .

RUN npm install -g tailwindcss

RUN npx tailwindcss -i ./input.css -o ./tailwind.css --minify

FROM jo3mccain/dioxus:latest AS builder

WORKDIR /app
ADD . .

COPY --from=node /app/tailwind.css /app/assets/tailwind.css

RUN dx build --release

FROM builder AS debug

ENV PORT=8080 \
    RUST_LOG=debug

EXPOSE ${PORT}

CMD serve --port 8080 --release

FROM nginx

ENV PORT=8080 \
    RUST_LOG=debug

COPY --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/nginx.conf /usr/local/nginx/conf/nginx.conf
COPY --from=builder /app/mime.types /usr/local/nginx/conf/mime.types