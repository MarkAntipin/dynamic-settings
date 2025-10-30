# Development

**Environment variables**
```
APPLICATION_PORT=18100	# The port on which the application (default: 18100)
API_KEY=api-key # The API key to access the API (default: api-key)
DATABASE_URL=sqlite://dynamic-settings.db # The database url (default: sqlite://dynamic-settings.db)
```

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
create .env file in the ui folder with:
```
VITE_API_URL=http://localhost:18100/api
```

**Install dependencies**
```
cd ui
npm install
```
**Run**
```
npm run dev
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
---
