#!/usr/bin/env bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --minify
dx build --release