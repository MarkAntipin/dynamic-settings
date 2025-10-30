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
COPY --from=frontend-builder /app/ui/dist /app/ui/dist

RUN mkdir -p /app/data

# Environment variables
ENV APP_DB_NAME=/app/data/dynamic-settings.db
ENV DATABASE_URL=sqlite:///app/data/dynamic-settings.db

# Run migrations on startup, then launch app
CMD ["./dynamic_settings"]
