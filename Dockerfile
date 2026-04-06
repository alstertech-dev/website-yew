# Stage 1: Build the Yew app
FROM rust:latest AS builder

# Install trunk and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .
RUN trunk build --release

# Stage 2: Serve with nginx
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
