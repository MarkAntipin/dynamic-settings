# Dynamic Settings Service


**Dependencies:**

- postgres
- golang-migrate (https://github.com/golang-migrate/migrate)

**Create .env with:**
```
APPLICATION_PORT=
PG_HOST=
PG_PORT=
PG_USERNAME=
PG_PASSWORD=
PG_DATABASE_NAME=
DATABASE_URL=
```

**Run postgres with docker:**
```
docker run --name postgres-dynamic-settings -e POSTGRES_USER=dynamic-settings -e POSTGRES_PASSWORD=dynamic-settings -e POSTGRES_DB=dynamic-settings -p 5436:5432 -d postgres
```

**Apply migrations:**
```
migrate -path migrations -database "postgres://dynamic-settings:dynamic-settings@localhost:5436/dynamic-settings?sslmode=disable" up
```

**Tests:**
```
cargo test
```

**Tests:**
```
cargo run
```

**Create migration:**
```
migrate create -ext sql -dir migrations {migration-name} 
```
