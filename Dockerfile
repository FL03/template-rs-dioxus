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

FROM nginx AS runner-base

ENV RUST_LOG=debug

# Copy source files
COPY --chown=755 --from=builder /workspace/dist /usr/share/nginx/html
COPY --chown=755 --from=node /workspace/tailwind.css /usr/share/nginx/html/tailwind.css
# Copy configuration files
COPY --from=workspace --chown=755 /src/.config/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=workspace --chown=755 /src/.config/nginx/mime.types /etc/nginx/mime.types
