FROM rust:1.83-alpine AS backend-builder

WORKDIR /usr/src/app

RUN apk add --no-cache \
    build-base \
    musl-dev \
    linux-headers \
    pkgconfig \
    openssl-dev

COPY . .

RUN cargo build --release --bin dynamic_settings

FROM node:18-alpine AS frontend-builder

WORKDIR /app/ui

COPY ui ./

RUN npm install && npm run build

FROM alpine

WORKDIR /app

COPY --from=backend-builder /usr/src/app/target/release/dynamic_settings .
COPY --from=frontend-builder /app/ui/dist /app/ui/dist

CMD ["./dynamic_settings"]
