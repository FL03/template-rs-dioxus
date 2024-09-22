#!/usr/bin/env bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --minify
rm Dioxus.toml
cp .config/Dioxus.pages.toml Dioxus.toml
dx build --release
mv docs/index.html docs/404.html