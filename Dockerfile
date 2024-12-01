# Builder stage
FROM --platform=linux/amd64 rust:1.74-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release

# Runtime stage
FROM --platform=linux/amd64 debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/eyeris /app/
COPY index.html /app/

ENV RUST_LOG=info
ENV PORT=8080
EXPOSE 8080

CMD ["./eyeris"] 