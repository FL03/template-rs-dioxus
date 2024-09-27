FROM scratch AS workspace

WORKDIR /src
# Copy the workspace
ADD . .

FROM nginx:latest AS debug

# Copy the nginx configuration
COPY --from=workspace --link /src/.config/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=workspace --link /src/.config/nginx/mime.types /etc/nginx/mime.types
# Copy the static files
COPY --chown=755 ./dist /usr/share/nginx/html

# Expose the port
EXPOSE 80

# Start the server
CMD ["nginx", "-g", "daemon off;"]