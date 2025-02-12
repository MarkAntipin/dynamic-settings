FROM rust:1.83-slim-bullseye AS backend-builder

WORKDIR /usr/src/app

COPY . .

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release --bin dynamic_settings

FROM node:18-bullseye-slim AS frontend-builder

WORKDIR /app/ui

COPY ui ./

RUN npm install && npm run build

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=backend-builder /usr/src/app/target/release/dynamic_settings .
COPY --from=frontend-builder /app/ui/dist /app/ui/dist

CMD ["./dynamic_settings"]