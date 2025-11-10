#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,sqlite"
  echo >&2 "to install it."
  exit 1
fi

# Default database file name (can be overridden with APP_DB_NAME)
APP_DB_NAME="${APP_DB_NAME:=dynamic-settings.db}"

# Ensure the directory exists
mkdir -p "$(dirname "$APP_DB_NAME")"

# Create SQLite database file if it doesn't exist
if [ ! -f "$APP_DB_NAME" ]; then
  echo "Creating SQLite database file: $APP_DB_NAME"
  touch "$APP_DB_NAME"
else
  echo "SQLite database already exists: $APP_DB_NAME"
fi

# Set the DATABASE_URL environment variable
export DATABASE_URL="sqlite://$APP_DB_NAME"

>&2 echo "Running migrations against $DATABASE_URL"

# Create tables and run migrations (from migrations/ folder)
sqlx database create 2>/dev/null || true
sqlx migrate run

>&2 echo "SQLite database has been migrated and is ready to go!"
