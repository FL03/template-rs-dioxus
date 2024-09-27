FROM scratch AS workspace

WORKDIR /src
ADD . .

FROM node:latest AS node

WORKDIR /workspace

COPY --from=workspace --chown=755 --link /src/input.css /workspace/input.css

RUN npm install -g tailwindcss

RUN npx tailwindcss -i ./input.css -o ./tailwind.css --minify

FROM jo3mccain/dioxus:latest AS builder

WORKDIR /workspace
ADD . .

COPY --chown=755 --from=node --link /workspace/tailwind.css /workspace/public/tailwind.css

RUN dx build --release

FROM nginx:latest AS runner-base

ENV NGINX_HOST=staging.scattered-systems.com \
    RUST_LOG=debug

EXPOSE 80

# Copy configuration files
COPY .config/nginx/nginx.conf /etc/nginx/nginx.conf
COPY .config/nginx/mime.types /etc/nginx/mime.types

# Copy source files
COPY --from=builder --link /workspace/dist /usr/share/nginx/html
COPY --from=builder --link /workspace/dist/assets/dioxus /usr/share/nginx/html/assets/dioxus
COPY --from=node --link /workspace/tailwind.css /usr/share/nginx/html/tailwind.css

FROM runner-base AS runner

CMD ["nginx", "-g", "daemon off;"]