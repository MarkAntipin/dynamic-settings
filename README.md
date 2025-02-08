# Dynamic Settings Service


**Run With docker:**
```
docker build -t dynamic-settings .
```

```
docker run -d -p 8000:8000 -e APPLICATION_PORT=8000 -e API_KEY=api-key -v db_data:/app/db dynamic-settings
```

**Create .env with:**
```
APPLICATION_PORT=
API_KEY=
```

**Linters:**
```
cargo clippy --all-targets --all-features
```

[//]: # (TODO: get rid of RUST_TEST_THREADS=1)
**Tests:**
```
RUST_TEST_THREADS=1 cargo test
```

**Run:**
```
cargo run
```
