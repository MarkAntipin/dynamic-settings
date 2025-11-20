# ⚙️️ Dynamic Settings

**Dynamic Settings** is a lightweight, real-time configuration management service written in rust
that allows you to store and update settings dynamically without redeploying your application.
It provides a web-based UI and an API to manage settings efficiently.

---

## 📖 Documentation

https://markantipin.github.io/dynamic-settings/

## 🚀 Quick setup
```
docker pull markantipin12/dynamic-settings

docker run -d -p 18100:18100 -v db_data:/app/db markantipin12/dynamic-settings

open http://localhost:18100
```

## 📚 API Documentation

The Swagger API documentation is available at `/docs` endpoint:
```
http://localhost:18100/docs
```

## ✨ Features

- **Real-time Configuration** – Modify settings in real-time without restarting your application
- **Built with Rust** – Designed for speed and safety
- **User-Friendly UI** – Manage settings through an intuitive web interface
- **API-Driven** – A RESTful API for programmatic access to settings


## 👷 Client Libraries
- **[Python](https://github.com/MarkAntipin/dynamic-settings-python)**

---

## 👨‍🏫 Tutorials
- **[Effortless Configuration: Making Your FastAPI App Dynamic](https://medium.com/@mr.antipin/effortless-configuration-making-your-fastapi-app-dynamic-d8652b0b4d56)**

---


## 🔧 Development

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
