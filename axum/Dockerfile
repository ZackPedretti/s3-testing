FROM rust:1.90 AS builder

# Compile
WORKDIR /app

COPY . .
RUN cargo build --release

# Runtime
FROM debian:bookworm-slim

WORKDIR /app
COPY --from=builder /app/target/release/axum /usr/local/bin/axum

CMD [ "axum" ]