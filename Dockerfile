# Stage 1: Build the Yew app
FROM rust:latest AS builder

# Install trunk and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .
RUN trunk build --release

FROM caddy:2.11-alpine AS final

# Copy built frontend from Stage 1
COPY --from=builder /app/dist /var/www/html

# Create Caddy configuration
COPY <<-"EOF" /etc/caddy/Caddyfile
:80 {
  # Static file server (frontend)
  handle {
    root * /var/www/html
    try_files {path} /index.html
    file_server
  }
}
EOF

EXPOSE 80