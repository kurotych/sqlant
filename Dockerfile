# Builder
FROM rust:1.83 AS builder
COPY . /app
WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runner
FROM alpine:3.21 
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sqlant /app/sqlant
WORKDIR /app

ENTRYPOINT ["/app/sqlant"]
CMD ["--help"]

