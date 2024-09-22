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

ARG GID=1000 \
    UID=1000 \
    GROUP=runners \
    USERNAME=runner

RUN groupadd -g ${GID} ${GROUP} && \
    useradd -g ${GROUP} -m -u ${UID} ${USERNAME}

# Switch to the custom user
USER ${USERNAME}

ENV RUST_LOG=debug

COPY --chmod=755 --from=builder /app/dist /usr/share/nginx/html
COPY --from=builder /app/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/nginx/mime.types /etc/nginx/conf/mime.types
