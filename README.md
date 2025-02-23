# Dynamic Settings Service

**Run With docker:**
```
docker build -t dynamic-settings .
```

```
docker run -d -p 18100:18100 -v db_data:/app/db dynamic-settings
```


```
APPLICATION_PORT=18100
API_KEY=api-key
```


## Development
### Backend
**Create .env with:**
```
APPLICATION_PORT=
API_KEY=
```

**Linters:**
```
cargo clippy --all-targets --all-features
```

**Tests:**
```
cargo test
```

**Run:**
```
cargo run
```
### Frontend
**Create .env with:**
```