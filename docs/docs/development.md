# Development

## With docker
```
docker build -t dynamic-settings .

docker run -d -p 18100:18100 dynamic-settings
```

## Without docker
### Backend
**Run**
```
cargo run
```
**Test**
```
cargo test
```
**Linter**
```
cargo clippy --all-targets --all-features
```

### Frontend (ui folder)
**Install dependencies**
```
npm install
```
**Run**
```
npm start
```
**Build**
```
npm run build
```

**Test**
```
npm test
```

**Linter**
```
npm run lint
```
