FROM rust:1.83-slim-bullseye AS builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release --bin dynamic_settings

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/dynamic_settings .

CMD ["./dynamic_settings"]