# Rust Microservice Template

This repository serves as a production-ready template for building scalable and maintainable microservices in Rust. It is built upon the principles of **Clean Architecture** (also known as Hexagonal or Ports and Adapters Architecture) to ensure a clear separation of concerns, making the codebase easy to test, evolve, and reason about.

## Getting Started

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

### 1. Configuration

The service is configured using environment variables. For local development, you can create a `.env` file in the project root.

Create a `.env` file by copying the example:

```sh
cp .env.example .env
```

Then, fill in the required variables in your new `.env` file:

```dotenv
# .env

# Server Configuration
PORT=8080

# Service Metadata for Tracing
SERVICE_NAME="my-hex-service"
PROJECT_ID="your-gcp-project-id" # Required for Google Cloud logging/tracing

# MongoDB Configuration
MONGO_URL="mongodb://localhost:27017"
MONGO_DB="my_database"

# Log Level (TRACE, DEBUG, INFO, WARN, ERROR)
DEBUG_LEVEL="INFO"
```

### 2. Running the Service

Once your `.env` file is configured, you can run the application with Cargo:

```sh
cargo run
```

The server will start, and you should see a log message indicating it is listening on the configured port.

## Key Features & Concepts

#### Log Levels

These are the log levels, ordered from least to most severe.

```
TRACE < DEBUG < INFO < WARN < ERROR
```

The `DEBUG_LEVEL` environment variable controls which log levels are displayed. By default, it is set to `INFO`, meaning that `INFO`, `WARN`, and `ERROR` logs will be visible. To see more detailed logs for debugging, set `DEBUG_LEVEL` to `DEBUG` or `TRACE`.

#### Example Usage

Simple text log:

```rust
tracing::info!("This is an info log");
tracing::warn!("This is a warning log");
```

Log with structured data, which is highly recommended for machine-readable logs:

```rust
tracing::info!(user_id = 42, "User logged in");
tracing::warn!(error = "Database connection failed", "A component failed to start");
```

### Optional Providers

The template includes several optional providers in `src/infrastructure/providers` for common Google Cloud services:

- `prv_pubsub.rs`: For publishing messages to Google Cloud Pub/Sub.
- `prv_storage.rs`: For interacting with Google Cloud Storage.
- `prv_tasks.rs`: For creating tasks in Google Cloud Tasks.

These are not wired into the application by default. To use them, you must expose them in `src/infrastructure/providers/mod.rs` and initialize them in `main.rs`, similar to how `MongoProvider` is handled.

## API Endpoints

This template provides a simple example feature with the following endpoints:

- `GET /api/v1/example`: Retrieves a list of all examples from the database.
- `GET /api/v1/example/error`: Retrieves an example that always returns an error, demonstrating error handling.
- `POST /api/v1/example/random`: Creates a new example with a random name and adds it to the database.
