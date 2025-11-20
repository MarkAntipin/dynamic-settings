# Getting started

**Dynamic Settings** is a lightweight, real-time configuration management service written in rust
that allows you to store and update settings dynamically without redeploying your application.
It provides a web-based UI and an API to manage settings efficiently.

## ✨ Features

- **Real-time Configuration** – Modify settings in real-time without restarting your application
- **Built with Rust** – Designed for speed and safety
- **User-Friendly UI** – Manage settings through an intuitive web interface
- **API-Driven** – A RESTful API for programmatic access to settings

## 🚀 Quick setup

```
docker pull markantipin12/dynamic-settings

docker run -e API_KEY=api-key -d -p 18100:18100 -v db_data:/app/db markantipin12/dynamic-settings

open http://localhost:18100
```

## 📚 API Documentation

The Swagger API documentation is available at `/docs` endpoint:
```
http://localhost:18100/docs
```

## 🙈 Environment Variables
```
APPLICATION_PORT=18100	# The port on which the application will run
API_KEY=api-key # The API key to access the API
DATABASE_URL=sqlite://dynamic-settings.db # The database url
```
