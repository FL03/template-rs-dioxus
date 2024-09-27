#! /usr/bin/env bash
docker run -d -i -p 80:80 --name template-rs-dioxus -v ./.config/nginx/nginx.conf:/etc/nginx/nginx.conf:ro -v ./.config/nginx/mime.types:/etc/nginx/mime.types:ro -v ./dist:/usr/share/nginx/html:ro nginx:latest
