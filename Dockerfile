# Dockerfile для продакшена Rust Auth
FROM rust:1.85.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/nanomed-mis-auth /app/nanomed-mis-auth
EXPOSE 8081
CMD ["/app/nanomed-mis-auth"]
