FROM jo3mccain/dioxus:latest as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN dioxus build --features web --platform web --release

FROM builder as development

ENV PORT=8080

EXPOSE ${PORT}

ENTRYPOINT [ "dioxus" ]
CMD [ "serve", "--release", "--port", "${PORT}" ]

FROM nginx:alpine-slim as production

COPY --from=builder /workspace/dist /usr/share/nginx/html