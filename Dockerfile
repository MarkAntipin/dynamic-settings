FROM rust:1.90-alpine AS backend-builder

WORKDIR /usr/src/app

RUN apk add --no-cache \
    build-base \
    musl-dev \
    linux-headers \
    pkgconfig \
    openssl-dev \
    sqlite-dev

COPY . .

RUN cargo build --release --bin dynamic_settings

FROM node:18-alpine AS frontend-builder

WORKDIR /app/ui

COPY ui ./

RUN npm install && npm run build

FROM alpine:latest

RUN apk add --no-cache bash sqlite

WORKDIR /app

COPY --from=backend-builder /usr/src/app/target/release/dynamic_settings .
COPY --from=backend-builder /usr/src/app/swagger /app/swagger
COPY --from=frontend-builder /app/ui/dist /app/ui/dist

RUN mkdir -p /app/db

ENV SQLITE_URL=sqlite:///app/db/dynamic-settings.db

# Run migrations on startup, then launch app
CMD ["./dynamic_settings"]
