#!/usr/bin/env bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch &
dx serve --hot-reload &
